#version 330 core

in vec3 vertexColor;
in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
    vec2 smileCoord = vec2(1.0 - texCoord.x, texCoord.y);
    FragColor = mix(texture(texture1, texCoord), texture(texture2, smileCoord), 0.2);
}
