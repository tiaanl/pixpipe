use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction};
use glium::{implement_vertex, program, uniform, Surface};
use pixpipe::{Color, PixBuf};
use std::borrow::Cow;

struct PixBufTexture2dDataSource<'a>(&'a PixBuf);

impl<'a> glium::texture::Texture2dDataSource<'a> for PixBufTexture2dDataSource<'a> {
    type Data = Color;

    fn into_raw(self) -> glium::texture::RawImage2d<'a, Self::Data> {
        glium::texture::RawImage2d {
            data: Cow::Owned(Vec::from(self.0.as_slice())),
            width: self.0.width(),
            height: self.0.height(),
            format: glium::texture::ClientFormat::U8U8U8U8,
        }
    }
}

#[derive(Debug)]
pub enum PipelineError {
    VertexBufferError(glium::vertex::BufferCreationError),
    IndexBufferError(glium::index::BufferCreationError),
    ProgramError(glium::program::ProgramChooserCreationError),
    TextureError(glium::texture::TextureCreationError),

    DrawError(glium::DrawError),
}

pub struct Pipeline {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,

    viewport_width: f32,
    viewport_height: f32,
}

impl Pipeline {
    pub fn new(display: &glium::Display) -> Result<Self, PipelineError> {
        Ok(Self {
            vertex_buffer: Self::create_vertex_buffer(display)?,
            index_buffer: Self::create_index_buffer(display)?,
            program: Self::create_program(display)?,

            viewport_width: 1.0,
            viewport_height: 1.0,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.viewport_width = width as f32;
        self.viewport_height = height as f32;
    }

    pub fn draw<F: glium::backend::Facade>(
        &self,
        facade: &F,
        target: &mut glium::Frame,
        pix_buf: &PixBuf,
    ) -> Result<(), PipelineError> {
        const SAMPLER_BEHAVIOUR: glium::uniforms::SamplerBehavior =
            glium::uniforms::SamplerBehavior {
                wrap_function: (
                    SamplerWrapFunction::Mirror,
                    SamplerWrapFunction::Mirror,
                    SamplerWrapFunction::Mirror,
                ),
                minify_filter: MinifySamplerFilter::Nearest,
                magnify_filter: MagnifySamplerFilter::Nearest,
                depth_texture_comparison: None,
                max_anisotropy: 1,
            };

        let texture = glium::texture::Texture2d::new(facade, PixBufTexture2dDataSource(pix_buf))
            .map_err(PipelineError::TextureError)?;

        let uniforms = uniform! {
            matrix: [
                [pix_buf.width() as f32 / self.viewport_width, 0.0, 0.0, 0.0],
                [0.0, pix_buf.height() as f32 / self.viewport_height, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: glium::uniforms::Sampler(&texture, SAMPLER_BEHAVIOUR),
        };

        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .map_err(PipelineError::DrawError)
    }

    fn create_vertex_buffer(
        display: &glium::Display,
    ) -> Result<glium::VertexBuffer<Vertex>, PipelineError> {
        glium::VertexBuffer::new(display, &VERTICES).map_err(PipelineError::VertexBufferError)
    }

    fn create_index_buffer<F: glium::backend::Facade>(
        facade: &F,
    ) -> Result<glium::IndexBuffer<u16>, PipelineError> {
        glium::IndexBuffer::new(
            facade,
            glium::index::PrimitiveType::TriangleStrip,
            &[1_u16, 2, 0, 3],
        )
        .map_err(PipelineError::IndexBufferError)
    }

    fn create_program<F: glium::backend::Facade>(
        facade: &F,
    ) -> Result<glium::Program, PipelineError> {
        program!(facade,
            140 => {
                vertex: "
                    #version 140
    
                    uniform mat4 matrix;
    
                    in vec2 position;
                    in vec2 tex_coords;
    
                    out vec2 v_tex_coords;
    
                    void main() {
                        gl_Position = matrix * vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                    }
                ",

                fragment: "
                    #version 140
                    uniform sampler2D tex;
                    in vec2 v_tex_coords;
                    out vec4 f_color;
    
                    void main() {
                        f_color = texture(tex, v_tex_coords);
                    }
                "
            },

            110 => {
                vertex: "
                    #version 110
    
                    uniform mat4 matrix;
    
                    attribute vec2 position;
                    attribute vec2 tex_coords;
    
                    varying vec2 v_tex_coords;
    
                    void main() {
                        gl_Position = matrix * vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                    }
                ",

                fragment: "
                    #version 110
                    uniform sampler2D tex;
                    varying vec2 v_tex_coords;
    
                    void main() {
                        gl_FragColor = texture2D(tex, v_tex_coords);
                    }
                ",
            },

            100 => {
                vertex: "
                    #version 100
    
                    uniform lowp mat4 matrix;
    
                    attribute lowp vec2 position;
                    attribute lowp vec2 tex_coords;
    
                    varying lowp vec2 v_tex_coords;
    
                    void main() {
                        gl_Position = matrix * vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                    }
                ",

                fragment: "
                    #version 100
                    uniform lowp sampler2D tex;
                    varying lowp vec2 v_tex_coords;
    
                    void main() {
                        gl_FragColor = texture2D(tex, v_tex_coords);
                    }
                ",
            },
        )
        .map_err(PipelineError::ProgramError)
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    const fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            tex_coords: [u, v],
        }
    }
}

const VERTICES: [Vertex; 4] = [
    Vertex::new(-1.0, -1.0, 0.0, 1.0),
    Vertex::new(-1.0, 1.0, 0.0, 0.0),
    Vertex::new(1.0, 1.0, 1.0, 0.0),
    Vertex::new(1.0, -1.0, 1.0, 1.0),
];

implement_vertex!(Vertex, position, tex_coords);
