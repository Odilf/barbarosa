# function testheuristic(heuristic; samples = 100)
# 	map(1:20) do i
# 		value = 0
# 		for _ in 1:samples
# 			value += heuristic(move(cube(), [rand(possible_moves) for _ in 1:i]))
# 		end
# 		value / samples
# 	end
# end

# using Plots
# function plot_heuristics()
# 	plot(testheuristic(manhattan); ylims=(0,4), label="manhattan")
# 	plot!(testheuristic(euclidean); label="euclidean")
# 	plot!(1:20, x -> x; label="\$ x = y \$")
# end

@test IDAstar(move(cube(), "R2 D2 F2"), manhattan; iterations=100, silent=true) |> reconstruct_solution == Cube3x3.parsealg("F2 D2 R2")
@test IDAstar(move(cube(), "R U R' U'"), manhattan; iterations=100, silent=true) |> reconstruct_solution == Cube3x3.parsealg("U R U' R'")