Select
    *
From
    Cinema
Where
    (id % 2) = 1
    and description != 'boring'
Order By
    rating desc