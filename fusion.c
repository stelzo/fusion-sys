#include "Fusion/Fusion/Fusion.h"

#include <stdlib.h>

void *create_ahrs()
{
    // alloc ahrs
    FusionAhrs *ahrs = (FusionAhrs *)malloc(sizeof(FusionAhrs));
    // initialise ahrs
    FusionAhrsInitialise(ahrs);
    return ahrs;
}

void free_ahrs(void *ahrs)
{
    free(ahrs);
}

void update_no_magnetometer(void *ahrs, double delta_time, double gyro_x, double gyro_y, double gyro_z, double accel_x, double accel_y, double accel_z)
{
    FusionVector gyroscope = {{gyro_x, gyro_y, gyro_z}};
    FusionVector accelerometer = {{accel_x, accel_y, accel_z}};
    FusionAhrsUpdateNoMagnetometer((FusionAhrs *)ahrs, gyroscope, accelerometer, delta_time);
}

void get_quaternion(void *ahrs, double *w, double *x, double *y, double *z)
{
    FusionQuaternion quaternion = FusionAhrsGetQuaternion((FusionAhrs *)ahrs);
    *w = quaternion.element.w;
    *x = quaternion.element.x;
    *y = quaternion.element.y;
    *z = quaternion.element.z;
}

void get_linear_acceleration(void *ahrs, double *x, double *y, double *z)
{
    FusionVector earth_acceleration = FusionAhrsGetLinearAcceleration((FusionAhrs *)ahrs);
    *x = earth_acceleration.axis.x;
    *y = earth_acceleration.axis.y;
    *z = earth_acceleration.axis.z;
}
