use std::ffi::{c_int,c_uint,c_void,c_char,c_float,CString};
use std::ptr;

type GLFWkeyfun=unsafe extern "C" fn(window:*mut c_void,key:c_int,scancode:c_int,action:c_int,mods:c_int);
//include/GLFW/glfw3.h:448:#define GLFW_KEY_ESCAPE 256
const GLFW_KEY_ESCAPE:c_int=256;
//include/GLFW/glfw3.h:338:#define GLFW_PRESS 1
const GLFW_PRESS:c_int=1;
//include/GL/glew.h:325:#define GL_FALSE 0
const GL_FALSE:i32=0;
//include/GL/glew.h:332:#define GL_TRUE 1
const GL_TRUE:i32=1;
//include/GL/glew.h:1780:#define GL_VERTEX_SHADER 0x8B31
const GL_VERTEX_SHADER:u32=0x8B31;
//include/GL/glew.h:1779:#define GL_FRAGMENT_SHADER 0x8B30
const GL_FRAGMENT_SHADER:u32=0x8B30;
//include/GL/glew.h:1659:#define GL_ARRAY_BUFFER 0x8892
const GL_ARRAY_BUFFER:u32=0x8892;
//include/GL/glew.h:1682:#define GL_STATIC_DRAW 0x88E4
const GL_STATIC_DRAW:u32=0x88E4;
//include/GL/glew.h:653:#define GL_FLOAT 0x1406
const GL_FLOAT:u32=0x1406;
//include/GL/glew.h:1660:#define GL_ELEMENT_ARRAY_BUFFER 0x8893
const GL_ELEMENT_ARRAY_BUFFER:u32=0x8893;
//include/GL/glew.h:341:#define GL_TRIANGLES 0x0004
const GL_TRIANGLES:u32=0x0004;
//include/GL/glew.h:652:#define GL_UNSIGNED_INT 0x1405
const GL_UNSIGNED_INT:u32=0x1405;
unsafe extern "C" fn key_callback(window:*mut c_void,key:c_int,scancode:c_int,action:c_int,mods:c_int){
   if key==GLFW_KEY_ESCAPE && action==GLFW_PRESS{
      glfwSetWindowShouldClose(window,GL_TRUE);
   }
}

#[link(name="GL")]
#[link(name="X11")]
#[link(name="m")]
#[link(name="glfw3",kind="static")]
#[link(name="GLEW",kind="static")]
unsafe extern "C"{
   fn glfwInit()->c_int;
   fn glfwTerminate();
   fn glfwCreateWindow(width:c_int,
                       height:c_int,
                       title:* const c_char,
                       monitor:* mut c_void,
                       share:* mut c_void)->* mut c_void;
   fn glfwMakeContextCurrent(window: *mut c_void);
   fn glewInit()->c_int;
   fn glfwGetFramebufferSize(
       window: *mut c_void,
       width: *mut c_int,
       height: *mut c_int
       );
   fn glViewport(x:c_int,y:c_int,width:isize,height:isize);
   fn glfwWindowShouldClose(window:*mut c_void)->c_int;
   fn glfwPollEvents();
   fn glClearColor(red:c_float,green:c_float,blue:c_float,alpha:c_float);
   fn glClear(mask:c_uint);
   fn glfwSwapBuffers(window:*mut c_void);
   fn glfwSetKeyCallback(window:*mut c_void,callback:GLFWkeyfun)->GLFWkeyfun;
   fn glfwSetWindowShouldClose(window:*mut c_void,value: c_int);
   //auto/core/gl/GL_VERSION_2_0:98: GLuint glCreateShader (GLenum type)
   fn glCreateShader(t:c_uint)->c_uint;
   fn glCreateProgram()->c_uint;
   //auto/core/gl/GL_VERSION_2_0:112:        void glShaderSource (GLuint shader, GLsizei count, const GLchar *const* string, const GLint* length)
   fn glShaderSource(shader:c_uint,count:isize,string:*const *const c_char,length:*const c_int);
   fn glCompileShader(shader:c_uint);
   fn glAttachShader(program:c_uint,shader:c_uint);
   fn glLinkProgram(program:c_uint);
   fn glDeleteShader(shader:c_uint);
   fn glDeleteProgram(program:c_uint);
   //void glDeleteBuffers(GLsizei n,const GLuint * buffers);
   fn glDeleteBuffers(n:isize,buffers:*const c_uint);
   //void glGenVertexArrays(GLsizei n,GLuint *arrays);
   fn glGenVertexArrays(n:isize,arrays:*mut c_uint);
   //void glGenBuffers(GLsizei n,GLuint * buffers);
   fn glGenBuffers(n:isize,buffers:*mut c_uint);
   //void glBindVertexArray(GLuint array);
   fn glBindVertexArray(array:c_uint);
   //void glBindBuffer(GLenum target,GLuint buffer);
   fn glBindBuffer(target:c_uint,buffer:c_uint);
   //void glBufferData(GLenum target,GLsizeiptr size,const void* data,GLenum usage);
   fn glBufferData(target:c_uint,size: isize,data:* const c_void,usage: c_uint);
   //void glVertexAttribPointer(GLuint index,GLint size,GLenum type,GLboolean normalized,
   //GLsizei stride,const void * pointer);
   fn glVertexAttribPointer(
       index:c_uint,
       size:c_uint,
       t:c_uint,
       normalized:u8,
       stride:c_int,
       pointer:* const c_void
       );
   //void glEnableVertexAttribArray(GLuint index);
   fn glEnableVertexAttribArray(index:c_uint);
   //void glDrawElements(GLenum mode,GLsizei count,GLenum type,const void * indices);
   fn glDrawElements(mode:c_uint,count:isize,t:c_uint,indices:*const c_void);
   //void glUseProgram(GLuint program);
   fn glUseProgram(program:c_uint);
}

const WIDTH:c_int=800;
const HEIGHT:c_int=600;
const GL_COLOR_BUFFER_BIT:c_uint=0x00004000;

fn main(){
   let mut is_ok:i32=0;
   unsafe{
      is_ok=glfwInit();
      if is_ok==GL_FALSE{
        glfwTerminate();
        panic!("glfwInit failed")
      }
      //https://stackoverflow.com/questions/29483365/what-is-the-syntax-for-a-multiline-string-literal
      let vertexShaderSource=r#"
         #version 330 core
         layout (location = 0) in vec3 position;
         void main()
         {
            gl_Position=vec4(position.x,position.y,position.z,1.0f);
         }
      "#;

      let fragementShaderSource=r#"
         #version 330 core
         out vec4 color;
         void main()
         {
            color=vec4(1.0f,0.5f,0.2f,1.0f);
         }
      "#;
      let window=glfwCreateWindow(WIDTH,
                                  HEIGHT,
                                  CString::new("hello from rust").unwrap().as_ptr() as *const c_char,
                                  ptr::null_mut(),
                                  ptr::null_mut()
                                  );
      glfwMakeContextCurrent(window);
      glfwSetKeyCallback(window,key_callback);
      glewInit();
      let mut width:c_int=0;
      let mut height:c_int=0;
      glfwGetFramebufferSize(window, &mut width, &mut height);
      glViewport(0,0,width as isize,height as isize);

      let vertexShader=glCreateShader(GL_VERTEX_SHADER);
      glShaderSource(vertexShader, 1, &(CString::new(vertexShaderSource).unwrap().as_ptr()), ptr::null());
      glCompileShader(vertexShader);
      let fragmentShader=glCreateShader(GL_FRAGMENT_SHADER);
      glShaderSource(fragmentShader, 1, &(CString::new(fragementShaderSource).unwrap().as_ptr()), ptr::null());
      glCompileShader(fragmentShader);
      let shaderProgram=glCreateProgram();
      glAttachShader(shaderProgram, vertexShader);
      glAttachShader(shaderProgram, fragmentShader);
      glLinkProgram(shaderProgram);
      glDeleteShader(vertexShader);
      glDeleteShader(fragmentShader);
      let vertices:[c_float;12]=[
         0.5, 0.5, 0.0,
         0.5,-0.5, 0.0,
        -0.5,-0.5, 0.0,
        -0.5, 0.5, 0.0
      ];
      let indices:[c_uint;6]=[
         0, 1, 3,
         1, 2, 3
      ];
      let mut VBO:c_uint=0;
      let mut VAO:c_uint=0;
      let mut EBO:c_uint=0;

      glGenVertexArrays(1,&mut (VAO));
      glGenBuffers(1,&mut (VBO));
      glGenBuffers(1,&mut (EBO));

      glBindVertexArray(VAO);
      glBindBuffer(GL_ARRAY_BUFFER,VBO);
      glBufferData(GL_ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize, vertices.as_ptr() as *const c_void, GL_STATIC_DRAW);

      glBindBuffer(GL_ELEMENT_ARRAY_BUFFER,EBO);
      glBufferData(GL_ELEMENT_ARRAY_BUFFER,std::mem::size_of_val(&indices) as isize,indices.as_ptr() as *const c_void,GL_STATIC_DRAW);
      glVertexAttribPointer(0, 3, GL_FLOAT ,GL_FALSE as u8, (3 * std::mem::size_of::<c_float>()) as c_int, std::ptr::null_mut());
      glEnableVertexAttribArray(0);
      glBindBuffer(GL_ARRAY_BUFFER,0);
      glBindVertexArray(0);
      while glfwWindowShouldClose(window)==GL_FALSE{
         glfwPollEvents();
         glClearColor(0.2,0.3,0.3,1.0);
         glClear(GL_COLOR_BUFFER_BIT);
         glUseProgram(shaderProgram);
         glBindVertexArray(VAO);
         glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT,  std::ptr::null());
         glBindVertexArray(0);
         glfwSwapBuffers(window);
      }
   }
}
