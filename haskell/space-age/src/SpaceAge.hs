module SpaceAge (Planet(..), ageOn) where

import Data.List
import Data.Maybe

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune deriving (Eq, Show)

earthYearInSeconds :: Float
earthYearInSeconds = 31557600

planetOrbits :: [(Planet, Float)]
planetOrbits = 
  [(Mercury, 0.2408467)
  ,(Venus, 0.61519726)
  ,(Earth, 1.0)
  ,(Mars, 1.8808158)
  ,(Jupiter, 11.862615)
  ,(Saturn, 29.447498)
  ,(Uranus, 84.016846)
  ,(Neptune, 164.79132)
  ]

convert :: Float -> Float -> Float
convert seconds  orbit = seconds / (orbit * earthYearInSeconds)

planetOrbit :: Planet -> Float
planetOrbit planet =
  let (_, orbit) = fromJust $ find (\(p, _) -> p == planet) planetOrbits in orbit

ageOn :: Planet -> Float -> Float
ageOn planet seconds = convert seconds $ planetOrbit planet
