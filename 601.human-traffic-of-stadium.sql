# Write your MySQL query statement below
/* Write your T-SQL query statement below */
-- get rank for stadium table with more than 100 people 
with a as (
    select
        id,
        visit_date,
        dense_rank() over (
            order by
                id asc
        ) as rn,
        people
    from
        stadium st
    where
        people >= 100
),
ab as (
    select
        (a.id - a.rn) as diff,
        count(*) as counter
    from
        a
    group by
        (a.id - a.rn)
    having
        count(*) >= 3
)
select
    a.id,
    a.visit_date,
    a.people
from
    a
    inner join ab on (a.id - a.rn) = ab.diff
order by
    a.id