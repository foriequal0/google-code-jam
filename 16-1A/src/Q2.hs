module Q2 (
    main
    ) where

import Data.List
import Data.Char as Char
import Control.Monad 

solve :: [Int] -> String
solve x =
    let g = group $ sort x
        a = zip (map head g) (map length g)
        f = filter (\(_, count) -> odd count) a
        h = map fst f
        s = sort h
    in intercalate " " $ map show s

main :: IO()
main = do
    t <- getInt
    forM_ [1..t] $ \i -> do
        n <- getInt
        x <- getInts $ n*(2*n-1)
        printCase i
        putStrLn $ solve x
    

takeWhileM :: (Monad m) => (a -> Bool) -> [m a] -> m [a]
takeWhileM _ [] = return []
takeWhileM p (x:xs) = do
    v <- x
    if p v
        then (v : ) <$> takeWhileM p xs
        else return []

getWords :: Int -> IO [String]
getWords x
    | x > 0 = do
          word <- takeWhileM (not . isSpace) $ repeat getChar
          (word :) <$> getWords (x-1)
    | otherwise = return []

getInts :: Int -> IO [Int]
getInts x = map read <$> getWords x

getInt :: IO Int
getInt = head <$> getInts 1

printCase :: Int -> IO ()
printCase x = do
    putStr $ "Case #" ++ (show x) ++ ": "
    
