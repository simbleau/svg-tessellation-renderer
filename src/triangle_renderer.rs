use crate::tessellator::artifacts::TessellationData;
use crate::tessellator::tessellator::LyonTessellator;

use super::error::{NaiveRendererError, Result};
use super::state::State;
use super::types::SceneGlobals;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub struct TriangleRenderer {
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    state: Option<State>,
}

impl TriangleRenderer {
    #[cfg(target_os = "windows")]
    fn get_event_loop() -> EventLoop<()> {
        winit::platform::windows::EventLoopExtWindows::new_any_thread()
    }

    #[cfg(target_os = "unix")]
    fn get_event_loop() -> EventLoop<()> {
        winit::platform::unix::EventLoopExtUnix::new_any_thread()
    }

    pub fn new() -> Self {
        TriangleRenderer {
            window: None,
            event_loop: None,
            state: None,
        }
    }

    pub fn init_with_svg<S: Into<String>>(
        &mut self,
        svg_contents: S,
    ) -> Result<()> {
        // Get contents
        let svg_contents = svg_contents.into();

        // Get global scene space
        let scene = super::util::get_globals(&svg_contents);

        // Tessellate the data
        let mut tessellator = LyonTessellator::new();
        tessellator.init(&svg_contents);
        let data = tessellator
            .tessellate()
            .map_err(|err| NaiveRendererError::FatalTessellationError(err))?;

        self.init(scene, data)
    }

    pub fn init(
        &mut self,
        scene: SceneGlobals,
        data: TessellationData,
    ) -> Result<()> {
        let event_loop_thread: EventLoop<()> =
            winit::platform::unix::EventLoopExtUnix::new_any_thread();
        let window = WindowBuilder::new().build(&event_loop_thread)?;
        window.set_resizable(true);
        window.set_title("Render-Kit");
        let state = pollster::block_on(State::new(&window, scene, data));

        self.window = Some(window);
        self.event_loop = Some(event_loop_thread);
        self.state = Some(state);

        Ok(())
    }

    pub fn toggle_wireframe(&mut self) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.toggle_wireframe();
        } else {
            return Err(NaiveRendererError::RendererNotInitialized);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if let (Some(state), Some(window), Some(event_loop)) = (
            self.state.as_mut(),
            self.window.as_mut(),
            self.event_loop.as_mut(),
        ) {
            event_loop.run_return(move |event, _, control_flow| match event {
                Event::RedrawRequested(_) => match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit
                    }
                    Err(e) => eprintln!("{:?}", e),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::Resized(size) => state.resize(size),
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => state.input(key),
                    _ => {}
                },
                _ => {}
            });
        } else {
            return Err(NaiveRendererError::RendererNotInitialized);
        }

        Ok(())
    }
}
