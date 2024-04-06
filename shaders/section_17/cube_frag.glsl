#version 330 core

#define POINT_LIGHT_COUNT 4

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct Spotlight {
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
uniform vec3 cameraPos;
uniform int spotlightEnabled;

uniform DirLight dirLight;
uniform PointLight pointLights[POINT_LIGHT_COUNT];
uniform Spotlight spotlight;

vec3 calculateDirectionLighting(DirLight dirLight, vec3 fragPos, vec3 camPos, vec3 norm) {
    vec3 lightDir = normalize(-dirLight.direction);
    vec3 cameraDir = normalize(camPos - fragPos);

    vec3 diffuseTexel = vec3(texture(material.diffuse, TexCoord));
    vec3 specularTexel = vec3(texture(material.specular, TexCoord));

    // Ambient lighting
    vec3 ambient = dirLight.ambient * diffuseTexel;

    // Diffuse lighting
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = dirLight.diffuse * diff * diffuseTexel;

    // Specular lighting
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(reflectDir, cameraDir), 0.0), material.shininess);
    vec3 specular = dirLight.specular * spec * specularTexel;

    return ambient + diffuse + specular;
}

vec3 calculatePointLighting(PointLight pointLight, vec3 fragPos, vec3 camPos, vec3 norm) {
    vec3 lightDir = normalize(pointLight.position - fragPos);
    vec3 cameraDir = normalize(camPos - fragPos);

    vec3 diffuseTexel = vec3(texture(material.diffuse, TexCoord));
    vec3 specularTexel = vec3(texture(material.specular, TexCoord));

    // Light strength attenuation
    float lightDist = distance(pointLight.position, fragPos);
    float attenuation = 1.0 / (pointLight.constant +
                pointLight.linear * lightDist +
                pointLight.quadratic * pow(lightDist, 2.0));

    // Ambient lighting
    vec3 ambient = pointLight.ambient * diffuseTexel;

    // Diffuse lighting
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = pointLight.diffuse * diff * diffuseTexel;

    // Specular lighting
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(reflectDir, cameraDir), 0.0), material.shininess);
    vec3 specular = pointLight.specular * spec * specularTexel;

    return (ambient + diffuse + specular) * attenuation;
}

vec3 calculateSpotlightLighting(Spotlight spotlight, vec3 fragPos, vec3 camPos, vec3 norm) {
    vec3 lightDir = normalize(spotlight.position - fragPos);
    vec3 cameraDir = normalize(camPos - fragPos);

    vec3 diffuseTexel = vec3(texture(material.diffuse, TexCoord));
    vec3 specularTexel = vec3(texture(material.specular, TexCoord));

    // Light strength attenuation
    float lightDist = distance(spotlight.position, fragPos);
    float attenuation = 1.0 / (spotlight.constant +
                spotlight.linear * lightDist +
                spotlight.quadratic * pow(lightDist, 2.0));

    // Spotlight effect
    float theta = dot(lightDir, normalize(-spotlight.direction));
    float intensity = clamp((theta - spotlight.outerCutoff) / (spotlight.innerCutoff - spotlight.outerCutoff), 0.0, 1.0);

    // Ambient lighting
    vec3 ambient = spotlight.ambient * diffuseTexel;

    // Diffuse lighting
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = spotlight.diffuse * diff * diffuseTexel;

    // Specular lighting
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(reflectDir, cameraDir), 0.0), material.shininess);
    vec3 specular = spotlight.specular * spec * specularTexel;

    return (ambient + diffuse + specular) * attenuation * intensity;
}

void main() {
    vec3 norm = normalize(Normal);
    vec3 result = vec3(0.0);

    // Directional lighting
    result += calculateDirectionLighting(dirLight, FragPos, cameraPos, norm);
    // Point lights
    for (int i = 0; i < POINT_LIGHT_COUNT; i++) {
        result += calculatePointLighting(pointLights[i], FragPos, cameraPos, norm);
    }
    // Spotlight
    if (bool(spotlightEnabled))
        result += calculateSpotlightLighting(spotlight, FragPos, cameraPos, norm);

    FragColor = vec4(result, 1.0);
}
