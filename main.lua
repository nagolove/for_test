
function foo(n)
    --print(n)
    if (n > 0) then
        foo(n - 1)
    end
    return 0
end

local ITER = 1000000
--local ITER = 100
foo(ITER)
