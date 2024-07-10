def cs(P):
	P = P.dual().relabel()
	m = P.join_matrix()

	n = len(P)
	class Frame:
		def __init__(self,n):
			self.next = 0 
			self.span = Bitset([0], capacity=n)
			self.gens = Bitset(capacity=n)

	stack = [Frame(n)]
	out = 0

	while stack:
		frame = stack.pop()

		for next in (next for next in range(frame.next, n) if not next in frame.span):
			# next is the next node to make a decision about.
			# First, decide no then push to stack.
			new_frame = deepcopy(frame)
			new_frame.next = next+1
			stack.append(new_frame)

			# Then, decide yes and keep processing
			frame.gens.add(next)
			for o in frame.span:
				frame.span.add(m[next, o])
		# No more decisions, so this is a minimal generating set.
		out += 1
	return out

#G = QuaternionGroup()
#subgroups = G.subgroups()
#f = lambda h,k: h.is_subgroup(k)
#P = LatticePoset((subgroups, f))

P = posets.BooleanLattice(2)

print(cs(P))

#length = 5
#for m1 in range(1,length+1):
#	for m2 in range(1,length+1):
#		for m3 in range(1,length+1):
#			sP = s(posets.ProductOfChains([m1,m2,m3]))
#			print(f'm1: {m1}, m2: {m2}, m3: {m3}, s: {sP}')
