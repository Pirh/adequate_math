use std::iter::Sum;
use std::ops::*;

use primitive::Primitive;
use as_tuple::AsTuple;


macro_rules! vector_type
{
    ($name: ident, $size: tt, $tuple: ident) => {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name<T: Copy>(pub [T; $size]);


        impl<T: Copy> $name<T>
        {
            #[inline(always)]
            pub fn map<F, U>(&self, operator: F) -> $name<U>
            where
                U: Copy,
                F: Fn(T) -> U
            {
                let mut result: $name<U> = unsafe { ::std::mem::uninitialized() };
                for i in 0..$size
                {
                    result.0[i] = operator(self.0[i]);
                }
                result
            }

            #[inline(always)]
            pub fn zipmap<F, U, V>(&self, other: $name<U>, operator: F) -> $name<V>
            where
                U: Copy,
                V: Copy,
                F: Fn(T, U) -> V
            {
                let mut result: $name<V> = unsafe { ::std::mem::uninitialized() };
                for i in 0..$size
                {
                    result.0[i] = operator(self.0[i], other.0[i]);
                }
                result
            }

            #[inline(always)]
            pub fn as_array(&self) -> [T; $size] { self.0 }

            #[inline(always)]
            pub fn as_tuple(&self) -> $tuple<T> { self.0.as_tuple() }
        }


        impl<T: Copy> $name<T>
        where
            T: Primitive
        {
            pub fn as_u8(&self) -> $name<u8> { self.map(|x| x.as_u8()) }
            pub fn as_u16(&self) -> $name<u16> { self.map(|x| x.as_u16()) }
            pub fn as_u32(&self) -> $name<u32> { self.map(|x| x.as_u32()) }
            pub fn as_u64(&self) -> $name<u64> { self.map(|x| x.as_u64()) }
            pub fn as_usize(&self) -> $name<usize> { self.map(|x| x.as_usize()) }
            pub fn as_i8(&self) -> $name<i8> { self.map(|x| x.as_i8()) }
            pub fn as_i16(&self) -> $name<i16> { self.map(|x| x.as_i16()) }
            pub fn as_i32(&self) -> $name<i32> { self.map(|x| x.as_i32()) }
            pub fn as_i64(&self) -> $name<i64> { self.map(|x| x.as_i64()) }
            pub fn as_isize(&self) -> $name<isize> { self.map(|x| x.as_isize()) }
            pub fn as_f32(&self) -> $name<f32> { self.map(|x| x.as_f32()) }
            pub fn as_f64(&self) -> $name<f64> { self.map(|x| x.as_f64()) }
        }

        impl<T> Add for $name<T>
        where
            T: Copy + Add<Output=T>
        {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output { self.zipmap(other, Add::add) }
        }

        impl<T> Sub for $name<T>
        where
            T: Copy + Sub<Output=T>
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output { self.zipmap(other, Sub::sub) }
        }

        impl<T> Mul for $name<T>
        where
            T: Copy + Mul<Output=T>
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output { self.zipmap(other, Mul::mul) }
        }

        impl<T> Div for $name<T>
        where
            T: Copy + Div<Output=T>
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output { self.zipmap(other, Div::div) }
        }

        impl<T> Mul<T> for $name<T>
        where
            T: Copy + Mul<Output=T>
        {
            type Output = Self;

            fn mul(self, other: T) -> Self::Output { self.map(|x| x * other) }
        }

        impl<T> Div<T> for $name<T>
        where
            T: Copy + Div<Output=T>
        {
            type Output = Self;

            fn div(self, other: T) -> Self::Output { self.map(|x| x / other) }
        }

        impl<T: Copy> AddAssign for $name<T>
        where
            Self: Add<Output=Self>
        {
            fn add_assign(&mut self, other: Self) { *self = *self + other }
        }

        impl<T: Copy> SubAssign for $name<T>
        where
            Self: Sub<Output=Self>
        {
            fn sub_assign(&mut self, other: Self) { *self = *self - other }
        }

        impl<T: Copy> MulAssign for $name<T>
        where
            Self: Mul<Output=Self>
        {
            fn mul_assign(&mut self, other: Self) { *self = *self * other }
        }

        impl<T: Copy> DivAssign for $name<T>
        where
            Self: Div<Output=Self>
        {
            fn div_assign(&mut self, other: Self) { *self = *self / other }
        }

        impl<T: Copy> MulAssign<T> for $name<T>
        where
            Self: Mul<T, Output=Self>
        {
            fn mul_assign(&mut self, other: T) { *self = *self * other }
        }

        impl<T: Copy> DivAssign<T> for $name<T>
        where
            Self: Div<T, Output=Self>
        {
            fn div_assign(&mut self, other: T) { *self = *self / other }
        }

        impl<T> Neg for $name<T>
        where
            T: Copy + Neg<Output=T>
        {
            type Output = Self;

            fn neg(self) -> Self { self.map(Neg::neg) }
        }

        impl<T> $name<T>
        where
            Self: Mul<Output=Self>,
            T: Copy + Sum<T>
        {
            pub fn dot(&self, other: Self) -> T { (*self * other).0.iter().cloned().sum() }

            pub fn mag_sq(&self) -> T { self.dot(*self) }
        }

        impl<T> $name<T>
        where
            Self: Mul<Output=Self> + Mul<T, Output=Self>,
            T: Copy + Sum<T> + Div<Output=T>
        {
            pub fn proj(&self, other: Self) -> Self { other * (self.dot(other) / other.dot(other)) }
        }

        impl $name<f32>
        {
            pub fn mag(&self) -> f32 { self.mag_sq().sqrt() }

            pub fn norm(&self) -> Self { *self / self.mag() }
        }

        impl $name<f64>
        {
            pub fn mag(&self) -> f64 { self.mag_sq().sqrt() }

            pub fn norm(&self) -> Self { *self / self.mag() }
        }
    }
}


type Tuple2<T> = (T, T);
type Tuple3<T> = (T, T, T);
type Tuple4<T> = (T, T, T, T);

vector_type!(Vec2, 2, Tuple2);
vector_type!(Vec3, 3, Tuple3);
vector_type!(Vec4, 4, Tuple4);


impl<T: Copy> Vec3<T>
where
    T: Mul<Output=T> + Sub<Output=T>
{
    pub fn cross(&self, other: Self) -> Self
    {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = other.as_tuple();
        Vec3([
             ay * bz - az * by,
             az * bx - ax * bz,
             ax * by - ay * bx])
    }
}


pub fn vec2<T: Copy>(x: T, y: T) -> Vec2<T>
{
    Vec2([x, y])
}

pub fn vec3<T: Copy>(x: T, y: T, z: T) -> Vec3<T>
{
    Vec3([x, y, z])
}

pub fn vec4<T: Copy>(x: T, y: T, z: T, w: T) -> Vec4<T>
{
    Vec4([x, y, z, w])
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn map()
    {
        let v2 = vec2(2, 3);
        let v3 = vec3(4, 5, 6);
        let v4 = vec4(7, 8, 9, 10);

        assert_eq!(v2.map(|x| x*x), vec2(4, 9));
        assert_eq!(v3.map(|x| x*x), vec3(16, 25, 36));
        assert_eq!(v4.map(|x| x*x), vec4(49, 64, 81, 100));
    }

    #[test]
    fn zipmap()
    {
        let a2 = vec2(2, 4);
        let b2 = vec2(3, 5);
        let a3 = vec3(1, 3, 5);
        let b3 = vec3(2, 4, 6);
        let a4 = vec4(2, 4, 6, 8);
        let b4 = vec4(1, 3, 5, 7);

        assert_eq!(a2.zipmap(b2, |x, y| x + y), vec2(5, 9));
        assert_eq!(a3.zipmap(b3, |x, y| x + y), vec3(3, 7, 11));
        assert_eq!(a4.zipmap(b4, |x, y| x + y), vec4(3, 7, 11, 15));
    }

    #[test]
    fn primitive_casts()
    {
        let f4 = vec4(1.0, 2.0, 3.0, 4.0);
        let i4 = vec4(1, 2, 3, 4);

        assert_eq!(f4.as_i32(), i4);
        assert_eq!(i4.as_f32(), f4);
    }

    #[test]
    fn as_tuple()
    {
        let (x, y, z, w) = vec4(0, 1, 2, 3).as_tuple();
        assert_eq!(x, 0);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
        assert_eq!(w, 3);
    }

    #[test]
    fn as_array()
    {
        let a = [10, 20, 30, 40];
        let v = Vec4(a);
        assert_eq!(v.as_array(), a);
    }

    #[test]
    fn binary_operators()
    {
        let u = vec4(2, 6, 9, 12);
        let v = vec4(2, 3, 3, 2);
        assert_eq!(u + v, vec4(4, 9, 12, 14));
        assert_eq!(u - v, vec4(0, 3, 6, 10));
        assert_eq!(u * v, vec4(4, 18, 27, 24));
        assert_eq!(u / v, vec4(1, 2, 3, 6));
    }

    #[test]
    fn scalar_operators()
    {
        let u = vec4(2, 6, 8, 12);
        assert_eq!(u * 2, vec4(4, 12, 16, 24));
        assert_eq!(u / 2, vec4(1, 3, 4, 6));
    }

    #[test]
    fn in_place_binary_operators()
    {
        let v = vec4(2, 3, 3, 2);
        let mut a = vec4(2, 6, 9, 12);
        let mut b = vec4(2, 6, 9, 12);
        let mut c = vec4(2, 6, 9, 12);
        let mut d = vec4(2, 6, 9, 12);
        a += v;
        b -= v;
        c *= v;
        d /= v;
        assert_eq!(a, vec4(4, 9, 12, 14));
        assert_eq!(b, vec4(0, 3, 6, 10));
        assert_eq!(c, vec4(4, 18, 27, 24));
        assert_eq!(d, vec4(1, 2, 3, 6));
    }

    #[test]
    fn in_place_scalar_operators()
    {
        let mut u = vec4(2, 6, 8, 12);
        let mut v = vec4(2, 6, 8, 12);
        u *= 2;
        v /= 2;
        assert_eq!(u, vec4(4, 12, 16, 24));
        assert_eq!(v, vec4(1, 3, 4, 6));
    }

    #[test]
    fn negation()
    {
        let v = vec4(-1, 2, -3, 4);
        assert_eq!(-v, vec4(1, -2, 3, -4));
    }

    #[test]
    fn dot_product()
    {
        let a = vec4(1, 0, 0, 0);
        let b = vec4(0, 1, 0, 0);
        let c = vec4(-1, 0, 0, 0);
        let d = vec4(1, 1, 0, 0);
        assert_eq!(a.dot(a), 1);
        assert_eq!(a.dot(b), 0);
        assert_eq!(a.dot(c), -1);
        assert_eq!(d.dot(d), 2);
    }

    #[test]
    fn mag_sq()
    {
        let a = vec4(1, 2, 3, 4);
        assert_eq!(a.mag_sq(), 30);
    }

    #[test]
    fn magnitude()
    {
        let v = vec2(3.0, 4.0_f64);
        assert_eq!(v.mag(), 5.0);
    }

    #[test]
    fn projection()
    {
        let v = vec3(2, 4, 6);
        let onto = vec3(1, 2, 0);
        assert_eq!(v.proj(onto), vec3(2, 4, 0));
    }

    #[test]
    fn normalization()
    {
        let v = vec4(10.0, 0.0, 0.0, 0.0_f32);
        assert_eq!(v.norm(), vec4(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn cross_product()
    {
        let x = vec3(1, 0, 0);
        let y = vec3(0, 1, 0);
        let z = vec3(0, 0, 1);
        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(x), -z);
        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(y), -x);
        assert_eq!(z.cross(x), y);
        assert_eq!(x.cross(z), -y);
    }
}
