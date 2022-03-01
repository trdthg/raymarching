#include <math.h>
#include <unistd.h>
#include <time.h>
#include <stdio.h>
#define WIDTH 40
#define HEIGHT 20
static char framebuffer[HEIGHT * WIDTH];

const int npixels = 7;
const char *pixels = " .:+|0#";

struct vec3
{
    float x;
    float y;
    float z;

    float length()
    {
        return sqrt(x * x + y * y + z * z);
    }

    void normalize()
    {
        float l = length();
        x = x / l;
        y = y / l;
        z = z / l;
    }

    struct vec3 operator*(float fac)
    {
        struct vec3 r;
        r.x = x * fac;
        r.y = y * fac;
        r.z = z * fac;
        return r;
    }

    struct vec3 operator+(struct vec3 other)
    {
        struct vec3 r;
        r.x = x + other.x;
        r.y = y + other.y;
        r.z = z + other.z;
        return r;
    }
    struct vec3 operator-(struct vec3 other)
    {
        struct vec3 r;
        r.x = x - other.x;
        r.y = y - other.y;
        r.z = z - other.z;
        return r;
    }
};

void raymarch();
float sdf(struct vec3);
char shade(struct vec3);

void raymarch()
{
    for (int y = 0; y < HEIGHT; y++)
    {
        for (int x = 0; x < WIDTH; x++)
        {
            struct vec3 pos = {0.0, 0.0, -3.0};
            struct vec3 target = {
                x / (float)WIDTH - 0.5f,
                (y / (float)HEIGHT - 0.5f) * (HEIGHT / (float)WIDTH) * 1.5f,
                -1.5f};

            struct vec3 ray = target - pos;
            ray.normalize();
            // printf("%lf  %lf  %lf\n", ray.x, ray.y, ray.z);
            char pxl = pixels[1];
            float dist;
            float max = 99.0f;
            for (int i = 0; i < 150; i++)
            {
                if (fabs(pos.x) > max || fabs(pos.y) > max || fabs(pos.z) > max)
                    break;

                dist = sdf(pos);
                if (dist < 1e-6)
                {
                    printf("%f \n", dist);
                    pxl = shade(pos);
                    break;
                }

                pos = pos + ray * dist;
                // if (pxl == '#')
                //     printf("%lf  %lf  %lf %f %d\n", pos.x, pos.y, pos.z, dist, pxl == '.');
            } // end for (i)

            framebuffer[y * WIDTH + x] = pxl;
        } // end for(x)
    }     // end for(y)
} // end raymarch()

float sdf(struct vec3 pos)
{
    struct vec3 center = {0.0, 0.0, 0.0};

    return (pos - center).length() - 0.2;
}

char shade(struct vec3 pos)
{
    return pixels[npixels - 1];
}

// Terminal clear sequence
const char *cls_seq = "\e[1;1H\e[2J";

void cls()
{
    write(0, cls_seq, 10);
}

void printfb()
{
    char *fb = framebuffer;
    char nl = '\n';
    cls();
    for (int y = 0; y < HEIGHT; y++)
    {
        write(1, fb, WIDTH);
        write(1, &nl, 1);
        fb += WIDTH;
    }
}

int main()
{
    for (int i = 0; i < WIDTH * HEIGHT; i++)
        framebuffer[i] = ' ';

    while (true)
    {
        raymarch();
        printfb();
        sleep(1);
    }
}