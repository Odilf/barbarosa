function reconstruct_solution(nodes::Vector{<:Cube})
	solution = []
	for (i, node) ∈ enumerate(nodes[2:end])
		for connection ∈ neighbouring_moves
			if move(node, connection.moves) == nodes[i]
				solution = [connection.moves..., solution...]
				break
			end

			error("Can't reconstruct solution. ")
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
		silent || println("Searching at depth $depth, threshold is $threshold")

		solution = search(cube, 0)
		if length(solution) != 0
			return solution
		end
		threshold = next_threshold
		next_threshold = Inf
	end

	error("Couldn't find solution after $iterations iterations")
end