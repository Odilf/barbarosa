# using .Cube3x3: possible_moves
# using Base.Threads

# function IDAstar(state::Cube{N}ube, heuristic; iterations = 100, silent = false)::Vector{Cube}
# 	h = heuristic(state)
# 	threshold = h
# 	next_threshold = Inf
# 	visited = Set()

# 	function search(node::Cube{N}ube, g)::Vector{Cube}
# 		if issolved(node)
# 			return [node::Cube{N}ube]
# 		end
	
# 		cost = heuristic(node) + g
# 		if cost > threshold
# 			# println("Exceded threshold ($threshold with cost $cost)")
# 			if cost < next_threshold
# 				next_threshold = cost
# 			end
# 			return []
# 		end
	
# 		for new_node ∈ neighbours(node)
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

# 	for depth ∈ 1:iterations
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

function reconstruct_solution(nodes::Vector{<:Cube})
	solution = []
	for (i, node) ∈ enumerate(nodes[2:end])
		for m ∈ Cube3x3.all_possible_moves
			if move(node, m) == nodes[i]
				solution = [m, solution...]
				break
			end
		end
	end
	solution
end


function IDAstar(cube::Cube{N}, heuristic; iterations = 100, silent = false)::Vector{Cube{N}} where N
	h = heuristic(cube)
	threshold = h
	next_threshold = Inf

	function search(node::Cube{N}, g)::Vector{Cube{N}}
		if issolved(node)
			return [node::Cube{N}]
		end
	
		cost = heuristic(node) + g
		if cost > threshold
			# println("Exceded threshold ($threshold with cost $cost)")
			if cost < next_threshold
				next_threshold = cost
			end
			return []
		end
	
		for connection ∈ neighbouring_moves
			result = search(move(node, connection.moves), g + connection.cost)
			if length(result) != 0
				return [result..., node]
			end
		end
	
		return []
	end

	for depth ∈ 1:iterations
		silent || println("Searching at depth $depth, threhsold is $threshold")

		solution = search(cube, 0)
		if length(solution) != 0
			return solution
		end
		threshold = next_threshold
		next_threshold = Inf
	end

	error("Couldn't find solution after $iterations iterations")
end