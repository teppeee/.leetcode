SELECT  e1.Name AS Employee
FROM Employee AS e1
JOIN Employee AS e2
ON e1.ManagerId = e2.Id
WHERE e1.Salary > e2.Salary 