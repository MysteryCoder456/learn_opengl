#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;

out vec4 VertColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 cameraPos;

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    vec3 FragPos = vec3(model * vec4(aPos, 1.0));
    vec3 Normal = transpose(inverse(mat3(model))) * aNormal;

    // Gourad Shading

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    vec3 cameraDir = normalize(cameraPos - FragPos);

    // Ambient lighting
    float ambientStrength = 0.1;
    vec3 ambient = vec3(ambientStrength);

    // Diffuse lighting
    vec3 diffuse = vec3(max(dot(norm, lightDir), 0.0));

    // Specular lighting
    float specularStrength = 0.5;
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = max(dot(reflectDir, cameraDir), 0.0);
    vec3 specular = vec3(pow(spec, 32) * specularStrength);

    vec3 result = (ambient + diffuse + specular) * objectColor * lightColor;
    VertColor = vec4(result, 1.0);
}
