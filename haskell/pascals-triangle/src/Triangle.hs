module Triangle (rows) where

triangles :: [[Integer]]
triangles = [[1], [1, 1], [1, 2, 1]]

rows :: Int -> [[Integer]]
rows x = 2 ^ (length rows)
