using .Cube3x3
using .Cube3x3: possible_moves

# function IDAstar(state::Cube, heuristic; iterations = 100, silent = false)::Vector{Cube}
# 	h = heuristic(state)
# 	threshold = h
# 	next_threshold = Inf
# 	visited = Set()

# 	function search(node::Cube, g)::Vector{Cube}
# 		if issolved(node)
# 			return [node::Cube]
# 		end
	
# 		cost = heuristic(node) + g
# 		if cost > threshold
# 			# println("Exceded threshold ($threshold with cost $cost)")
# 			if cost < next_threshold
# 				next_threshold = cost
# 			end
# 			return []
# 		end
	
# 		for new_node in neighbours(node)
# 			if new_node ∈ visited
# 				println("Skipping cause its visited")
# 				continue
# 			end

# 			println("Not skipping")

# 			result = search(new_node, g + 1)
# 			if length(result) != 0
# 				return [result..., node]
# 			else
# 				push!(visited, result)
# 			end
# 		end
	
# 		return []
# 	end

# 	for depth in 1:iterations
# 		silent || println("Searching at depth $depth")
# 		solution = search(state, 0)
# 		if length(solution) != 0
# 			return solution
# 		end
# 		threshold = next_threshold
# 		next_threshold = Inf
# 	end

# 	error("Couldn't find solution after $iterations iterations")
# end

function reconstruct_solution(nodes::Vector{Cube})
	solution = []
	for (i, node) in enumerate(nodes[2:end])
		for m in possible_moves
			if move(node, m) == nodes[i]
				solution = [m, solution...]
				break
			end
		end
	end
	solution
end




function IDAstar(state::Cube, heuristic; iterations = 100, silent = false)::Vector{Cube}
	h = heuristic(state)
	threshold = h
	next_threshold = Inf

	function search(node::Cube, g)::Vector{Cube}
		if issolved(node)
			return [node::Cube]
		end
	
		cost = heuristic(node) + g
		if cost > threshold
			# println("Exceded threshold ($threshold with cost $cost)")
			if cost < next_threshold
				next_threshold = cost
			end
			return []
		end
	
		for new_node in neighbours(node)
			result = search(new_node, g + 1)
			if length(result) != 0
				return [result..., node]
			end
		end
	
		return []
	end

	for depth in 1:iterations
		silent || println("Searching at depth $depth")

		solution = search(state, 0)
		if length(solution) != 0
			return solution
		end
		threshold = next_threshold
		next_threshold = Inf
	end

	error("Couldn't find solution after $iterations iterations")
end