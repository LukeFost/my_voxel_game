#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[link(name = "OpenGL", kind = "framework")]
extern "C" {}

// Standard library imports
use std::ptr;
use std::mem;
use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_long, c_uchar, c_short, c_ulong, c_ushort};

// Windows types and constants
type HDC = *mut c_void;
type HGLRC = *mut c_void;
type HINSTANCE = *mut c_void;
type HWND = *mut c_void;
type HCURSOR = *mut c_void;
type HICON = *mut c_void;
type HBRUSH = *mut c_void;
type LPCSTR = *const i8;
type LPVOID = *mut c_void;
type LRESULT = isize;
type WPARAM = usize;
type LPARAM = isize;
type UINT = u32;

const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
const CS_OWNDC: u32 = 0x0020;
const CS_HREDRAW: u32 = 0x0002;
const CS_VREDRAW: u32 = 0x0001;
const WS_OVERLAPPEDWINDOW: u32 = 0xcf0000;
const SW_SHOW: c_int = 5;
const PFD_TYPE_RGBA: u8 = 0;
const PFD_MAIN_PLANE: i8 = 0;
const PFD_DOUBLEBUFFER: u32 = 0x0001;
const PFD_DRAW_TO_WINDOW: u32 = 0x0004;
const PFD_SUPPORT_OPENGL: u32 = 0x00020;
const WM_DESTROY: UINT = 0x0002;
const WM_CLOSE: UINT = 0x0010;
const WM_QUIT: UINT = 0x0012;

#[repr(C)]
struct PIXELFORMATDESCRIPTOR {
    nSize: c_ushort,
    nVersion: c_ushort,
    dwFlags: u32,
    iPixelType: u8,
    cColorBits: u8,
    cRedBits: u8,
    cRedShift: u8,
    cGreenBits: u8,
    cGreenShift: u8,
    cBlueBits: u8,
    cBlueShift: u8,
    cAlphaBits: u8,
    cAlphaShift: u8,
    cAccumBits: u8,
    cAccumRedBits: u8,
    cAccumGreenBits: u8,
    cAccumBlueBits: u8,
    cAccumAlphaBits: u8,
    cDepthBits: u8,
    cStencilBits: u8,
    cAuxBuffers: u8,
    iLayerType: u8,
    bReserved: u8,
    dwLayerMask: u32,
    dwVisibleMask: u32,
    dwDamageMask: u32,
}

// WNDCLASS structure
#[repr(C)]
struct WNDCLASSA {
    style: u32,
    lpfnWndProc: Option<extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
}

// Windows API function declarations
extern "system" {
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HINSTANCE;
    fn RegisterClassA(lpWndClass: *const WNDCLASSA) -> u16;
    fn CreateWindowExA(
        dwExStyle: u32,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: u32,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HWND,
        hInstance: HINSTANCE,
        lpParam: LPVOID
    ) -> HWND;
    fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> c_int;
    fn UpdateWindow(hWnd: HWND) -> c_int;
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn DispatchMessageA(lpmsg: *const MSG) -> LRESULT;
    fn TranslateMessage(lpmsg: *const MSG) -> c_int;
    fn PeekMessageA(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin:UINT, wMsgFilterMax:UINT, wRemoveMsg:UINT) -> c_int;
    fn PostQuitMessage(nExitCode: c_int);
    fn GetDC(hWnd: HWND) -> HDC;
    fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    fn SetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    fn wglCreateContext(hdc: HDC) -> HGLRC;
    fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> c_int;
    fn SwapBuffers(hdc: HDC) -> c_int;
}

// Message struct
#[repr(C)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: u32,
    pt: POINT,
}

#[repr(C)]
struct POINT {
    x: c_long,
    y: c_long,
}


extern "system" {}

// OpenGL function imports
extern "system" {
    fn glClear(mask: u32);
    fn glBegin(mode: u32);
    fn glEnd();
    fn glVertex3f(x: f32, y: f32, z: f32);
    fn glColor3f(r: f32, g: f32, b: f32);
    fn glRotatef(angle: f32, x: f32, y: f32, z: f32);
    fn glTranslatef(x: f32, y: f32, z: f32);
    fn glMatrixMode(mode: u32);
    fn glLoadIdentity();
    fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, zNear: f64, zFar: f64);
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
}

// GL constants
const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
const GL_TRIANGLES: u32 = 0x0004;
const GL_QUADS: u32 = 0x0007;
const GL_PROJECTION: u32 = 0x1701;
const GL_MODELVIEW: u32 = 0x1700;

static mut RUNNING: bool = true;
static CLASS_NAME: &[u8] = b"RustGLWindowClass\0";
static WINDOW_NAME: &[u8] = b"Rust OpenGL Cube\0";

extern "system" fn WndProc(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY | WM_CLOSE => {
            unsafe {
                RUNNING = false;
            }
            unsafe { PostQuitMessage(0); }
            0
        }
        _ => unsafe { DefWindowProcA(hWnd, msg, wParam, lParam) }
    }
}

fn main() {
    unsafe {
        let hInstance = GetModuleHandleA(ptr::null());

        let wc = WNDCLASSA {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(WndProc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance,
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: CLASS_NAME.as_ptr() as LPCSTR,
        };

        if RegisterClassA(&wc) == 0 {
            eprintln!("Failed to register window class");
            return;
        }

        let hWnd = CreateWindowExA(
            0,
            CLASS_NAME.as_ptr() as LPCSTR,
            WINDOW_NAME.as_ptr() as LPCSTR,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT,
            800, 600,
            ptr::null_mut(),
            ptr::null_mut(),
            hInstance,
            ptr::null_mut()
        );

        if hWnd.is_null() {
            eprintln!("Failed to create window");
            return;
        }

        ShowWindow(hWnd, SW_SHOW);
        UpdateWindow(hWnd);

        let hDC = GetDC(hWnd);
        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: mem::size_of::<PIXELFORMATDESCRIPTOR>() as c_ushort,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 24,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 0,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 8,
            cAuxBuffers: 0,
            iLayerType: PFD_MAIN_PLANE as u8,
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        };

        let iFormat = ChoosePixelFormat(hDC, &pfd);
        SetPixelFormat(hDC, iFormat, &pfd);

        let hRC = wglCreateContext(hDC);
        wglMakeCurrent(hDC, hRC);

        // Basic OpenGL setup
        glClearColor(0.2, 0.3, 0.3, 1.0);

        // Setup perspective
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glFrustum(-1.0, 1.0, -0.75, 0.75, 1.0, 100.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();

        let mut angle: f32 = 0.0;

        while RUNNING {
            // Process messages
            let mut msg = MSG {
                hwnd: ptr::null_mut(),
                message: 0,
                wParam: 0,
                lParam: 0,
                time: 0,
                pt: POINT { x:0, y:0 },
            };

            while PeekMessageA(&mut msg, ptr::null_mut(), 0, 0, 1) != 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            // Render
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            glLoadIdentity();
            glTranslatef(0.0, 0.0, -5.0);
            glRotatef(angle, 1.0, 1.0, 0.0);

            // Draw a cube using GL_QUADS
            // Front face
            glBegin(GL_QUADS);
            glColor3f(1.0, 0.0, 0.0); // Red
            glVertex3f(-1.0, -1.0, 1.0);
            glVertex3f(1.0, -1.0, 1.0);
            glVertex3f(1.0, 1.0, 1.0);
            glVertex3f(-1.0, 1.0, 1.0);
            glEnd();

            // Back face
            glBegin(GL_QUADS);
            glColor3f(0.0, 1.0, 0.0); // Green
            glVertex3f(-1.0, -1.0, -1.0);
            glVertex3f(-1.0, 1.0, -1.0);
            glVertex3f(1.0, 1.0, -1.0);
            glVertex3f(1.0, -1.0, -1.0);
            glEnd();

            // Left face
            glBegin(GL_QUADS);
            glColor3f(0.0, 0.0, 1.0); // Blue
            glVertex3f(-1.0, -1.0, -1.0);
            glVertex3f(-1.0, -1.0, 1.0);
            glVertex3f(-1.0, 1.0, 1.0);
            glVertex3f(-1.0, 1.0, -1.0);
            glEnd();

            // Right face
            glBegin(GL_QUADS);
            glColor3f(1.0, 1.0, 0.0); // Yellow
            glVertex3f(1.0, -1.0, -1.0);
            glVertex3f(1.0, 1.0, -1.0);
            glVertex3f(1.0, 1.0, 1.0);
            glVertex3f(1.0, -1.0, 1.0);
            glEnd();

            // Top face
            glBegin(GL_QUADS);
            glColor3f(1.0, 0.0, 1.0); // Magenta
            glVertex3f(-1.0, 1.0, -1.0);
            glVertex3f(-1.0, 1.0, 1.0);
            glVertex3f(1.0, 1.0, 1.0);
            glVertex3f(1.0, 1.0, -1.0);
            glEnd();

            // Bottom face
            glBegin(GL_QUADS);
            glColor3f(0.0, 1.0, 1.0); // Cyan
            glVertex3f(-1.0, -1.0, -1.0);
            glVertex3f(1.0, -1.0, -1.0);
            glVertex3f(1.0, -1.0, 1.0);
            glVertex3f(-1.0, -1.0, 1.0);
            glEnd();

            SwapBuffers(hDC);

            angle += 1.0;
        }
    }
}
