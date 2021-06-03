Select
    Department,
    Employee,
    Salary
From
(
        select
            d.Name as Department,
            e.Name as Employee,
            Salary,
            DENSE_RANK() OVER (
                PARTITION BY d.Id
                ORDER BY
                    Salary DESC
            ) AS RankNo
        from
            Employee as e
            join Department as d on e.DepartmentId = d.Id
    ) as ed
where
    RankNo < 4