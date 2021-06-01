SELECT  q.Department
       ,e.Name AS Employee
       ,q.Salary
FROM Employee e
INNER JOIN 
(
	SELECT  d.Name      AS Department
	       ,D.Id
	       ,MAX(Salary) AS Salary
	FROM Employee e
	INNER JOIN Department d
	ON e.DepartmentId = d.Id
	GROUP BY  d.Name
	         ,d.Id
) q
ON q.Salary=e.Salary AND q.Id = e.DepartmentId 