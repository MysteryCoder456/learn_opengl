#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 direction;
    float innerCutoff;
    float outerCutoff;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

out vec4 FragColor;

uniform Material material;
uniform Light light;
uniform vec3 cameraPos;

void main() {
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    vec3 cameraDir = normalize(cameraPos - FragPos);

    vec3 diffuseTexel = vec3(texture(material.diffuse, TexCoord));
    vec3 specularTexel = vec3(texture(material.specular, TexCoord));

    // Light strength attenuation
    float lightDist = distance(light.position, FragPos);
    float attenuation = 1.0 / (light.constant +
                light.linear * lightDist +
                light.quadratic * pow(lightDist, 2.0));

    // Spotlight effect
    float theta = dot(lightDir, normalize(-light.direction));
    float intensity = clamp((theta - light.outerCutoff) / (light.innerCutoff - light.outerCutoff), 0.0, 1.0);

    // Ambient lighting
    vec3 ambient = light.ambient * diffuseTexel;

    // Diffuse lighting
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * diff * diffuseTexel;

    // Specular lighting
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(reflectDir, cameraDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * specularTexel;

    vec3 result = (ambient + diffuse + specular) * attenuation * intensity;
    FragColor = vec4(result, 1.0);
}
