#version 330 core

in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;

void main() {
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 norm = normalize(Normal);
    FragColor = vec4(objectColor * lightColor * dot(norm, lightDir), 1.0);
}
