module Clock (addDelta, fromHourMin, toString) where

data Clock = Clock 
  {
    hours :: Int,
    minutes :: Int
  } deriving Eq

-- format :: Clock -> String
-- format clock =
--   let mins = (hours clock) * 60 + (minutes clock)


fromHourMin :: Int -> Int -> Clock
fromHourMin hour mins = Clock hour mins

toString :: Clock -> String
toString clock = show (hours clock) ++ ":" ++ show (minutes clock)

addDelta :: Int -> Int -> Clock -> Clock
addDelta hour mins clock = Clock ((hours clock) + hour) ((minutes clock) + mins)

instance Show Clock where
  show = toString

