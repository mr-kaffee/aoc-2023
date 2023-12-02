import Data.Char (isDigit, ord)

main = do
  content <- readFile "../../../inputs/input02"
  putStrLn $ "The solution to star 1 is " ++ show (star1 content)
  putStrLn $ "The solution to star 2 is " ++ show (star2 content)

-- tag::star1[]
-- sum over ids of all lines which respect the limits
star1 :: String -> Int
star1 = sum . map fst . filter (allSmaller 12 13 14 . words . snd) . map splitId . lines
  where
    -- essentially a foldl which consumes two elements (count and color) at once
    allSmaller :: Int -> Int -> Int -> [String] -> Bool
    allSmaller _ _ _ [] = True
    allSmaller r g b (n : c : xs)
      | read n > choose r g b c = False
      | otherwise = allSmaller r g b xs

-- end::star1[]

-- tag::star2[]
-- sum of product of maximum for every color for every line
star2 :: String -> Int
star2 = sum . map (prod' . maxByColor (0, 0, 0) . words . snd . splitId) . lines
  where
    prod' :: (Int, Int, Int) -> Int
    prod' (r, g, b) = r * g * b
    -- essentially a foldl which consumes two elements (count and color) at once
    maxByColor :: (Int, Int, Int) -> [String] -> (Int, Int, Int)
    maxByColor (r, g, b) [] = (r, g, b)
    maxByColor (r, g, b) (n' : c : xs) =
      let n = read n'
       in case choose 1 2 3 c of
            1 -> maxByColor (max r n, g, b) xs
            2 -> maxByColor (r, max g n, b) xs
            3 -> maxByColor (r, g, max b n) xs

-- end::star2[]

-- tag::helpers[]
-- depending on the fourth argument, return one of the first three
-- to choose, only the first letter of the fourth argument is considered,
-- which is expected to be one of 'r' (return first), 'g' (return second),
-- or 'b' (return third)
choose :: a -> a -> a -> String -> a
choose r g b ('r' : _) = r
choose r g b ('g' : _) = g
choose r g b ('b' : _) = b

-- extract Game id: ...draws... into (id, ...draws...)
splitId :: String -> (Int, String)
splitId line@(x : xs)
  | isDigit x = splitId' 0 line
  | otherwise = splitId xs
  where
    splitId' :: Int -> String -> (Int, String)
    splitId' id (x : xs)
      | isDigit x = splitId' (10 * id + (ord x - ord '0')) xs
    splitId' id (':' : ' ' : xs) = (id, xs)

-- end::helpers[]
