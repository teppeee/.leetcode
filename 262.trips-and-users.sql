select
    Request_at as Day,
    cast(
        sum(
            case
                when Status != 'completed' then 1.0
                else 0.0
            end
        ) / (
            sum(
                case
                    when Status != 'completed' then 1.0
                    else 0.0
                end
            ) + sum(
                case
                    when Status = 'completed' then 1.0
                    else 0.0
                end
            )
        ) as decimal(3, 2)
    ) as [Cancellation Rate]
from
    Trips t
    inner join Users c on c.Users_Id = t.Client_Id
    and c.Banned = 'No'
    inner join Users d on d.Users_Id = t.Driver_Id
    and d.Banned = 'No'
where
    Request_at between '2013-10-01'
    and '2013-10-03'
group by
    Request_at