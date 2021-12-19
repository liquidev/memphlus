#version 150

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
uniform sampler2D u_palettes;
uniform vec4 u_diffuse;

out vec4 o_color;

void main() {
   // Sample the pixel. We'll figure out which color to use based on its channels.
   vec4 pixel = texture(u_texture, v_uv);

   // Figure out which palette we're using.
   float palette_index = pixel.b;
   vec4 background = texture(u_palettes, vec2(palette_index, 0.0));
   vec4 foreground = texture(u_palettes, vec2(palette_index, 0.25));
   vec4 accent = texture(u_palettes, vec2(palette_index, 0.5));

   // Calculate the final color based on the red and green channels.
   vec4 background_foreground = mix(background, foreground, pixel.r);
   vec4 accented = mix(background_foreground, accent, pixel.g);

   o_color = accented;
}
