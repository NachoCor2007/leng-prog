module Shape where

data Point = Point { x::Double, y:: Double} deriving (Eq, Show)

data Circle    = Circle    Point Double deriving (Eq, Show)
data Rectangle = Rectangle Point Point deriving (Eq, Show)


-- A point from a tuple Pair
point::(Double, Double) -> Point
point (newX, newY) = Point {x=newX, y=newY}

-- The origin
origin::Point
origin = Point {x=0.0, y=0.0}

-- Rectangle from a Tuple where (x0 y0) == origin
rectangle::(Double, Double) -> Rectangle
rectangle (newX, newY) = Rectangle origin Point {x=newX, y=newY}

base::Rectangle -> Double
base (Rectangle p0 p1) = x p1 - x p0

height::Rectangle -> Double
height (Rectangle p0 p1) = y p1 - y p0

-- Circle from radius
circle::Double -> Circle
circle r = Circle origin r

-- Clase Shift

class Shift a where
   shift::a -> (Double, Double) -> a
   
instance Shift Point where
   shift p (xShift, yShift) = Point (x p + xShift) (y p + yShift)
   
instance Shift Rectangle where
   shift (Rectangle p0 p1) xyShift = Rectangle (shift p0 xyShift) (shift p1 xyShift)
   
instance Shift Circle where
   shift (Circle p r) xyShift = Circle (shift p xyShift) r
   
-- Define the Surface class
class Surface a where
  surface::a -> Double

instance Surface Rectangle where
  surface r = base r * height r

instance Surface Circle where
  surface (Circle _ r) = pi * r ^ 2
