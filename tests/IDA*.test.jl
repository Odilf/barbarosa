using Test

include("../3x3/main.jl")
using .Cube3x3

include("../search/heuristics.jl")
include("../search/IDA*.jl")

function testheuristic(heuristic; samples = 100)
	map(1:20) do i
		value = 0
		for _ in 1:samples
			value += heuristic(move(cube(), [rand(possible_moves) for _ in 1:i]))
		end
		value / samples
	end
end

using Plots
let 
	plot(testheuristic(manhattan); ylims=(0,4), label="manhattan")
	plot!(testheuristic(euclidean); label="euclidean")
	plot!(1:20, x -> x; label="\$ x = y \$")
end


@benchmark IDAstar(move(cube(), "R2 L2 D2 F2"), manhattan; iterations=100, silent=true)
@benchmark IDAstarNotVisited(move(cube(), "R2 L2 D2 F2"), manhattan; iterations=100, silent=true)

IDAstar(move(cube(), "R2 L2 D2 F2"), manhattan; iterations=100, silent=false)