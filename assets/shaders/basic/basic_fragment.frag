#version 330 core
out vec4 FragColor;

void main()
{
	// linearly interpolate between both textures (80% texture 1, 20% texture 2)
	FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}