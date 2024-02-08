#version 330 core

in vec3 FragPos;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;

void main() {
    // Ambient lighting
    float ambientStrength = 0.1;
    vec3 ambient = lightColor * ambientStrength;

    // Diffuse lighting
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 norm = normalize(Normal);
    vec3 diffuse = lightColor * max(dot(norm, lightDir), 0.0);

    vec3 result = (ambient + diffuse) * objectColor;
    FragColor = vec4(result, 1.0);
}
