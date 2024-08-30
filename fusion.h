#ifndef FUSION_H
#define FUSION_H

void *create_ahrs();

void free_ahrs(void *ahrs);

void update_no_magnetometer(void *ahrs, double delta_time, double gyro_x, double gyro_y, double gyro_z, double accel_x, double accel_y, double accel_z);

void get_quaternion(void *ahrs, double *w, double *x, double *y, double *z);

void get_linear_acceleration(void *ahrs, double *x, double *y, double *z);

#endif // FUSION_H