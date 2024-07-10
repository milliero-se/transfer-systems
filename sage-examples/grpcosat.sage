G = SymmetricGroup(4)
tom = matrix(G.gap().TableOfMarks().MatTom())
N = tom.nrows()

# tom^i_j = |Hom(G/H_j, G/H_i)| = X(G/H_i)(H_j)
# H_i is subconjugate to H_j <==> |Hom(G/H_i,G/H_j)| > 0 <==> tom^i_j > 0 ==> i <= j.
# tom_ij > 0 ==> i >= j

# X(\sum_i n_i G/H_i)(H_j) = \sum_i n_i X(G/H_i)(H_j) = \sum_i n_i t^i_j = (n * tom)_j

ds = [[ tom.row(i).pairwise_product(tom.row(j)) / tom for j in range(0,N) ] for i in range(0,N)]
ds = list(map(lambda r: list(map(lambda v: [k for k in range(0,N) if v[k] != 0], r)), ds))

class Frame:
	def __init__(self,n):
		self.next = n-1
		self.span = Bitset([n-1], capacity=n)
		self.gens = Bitset(capacity=n)

stack = [Frame(N)]
out = []

while stack:
	frame = stack.pop()

	for next in (next for next in range(frame.next, -1,-1) if not next in frame.span):
		# next is the next node to make a decision about.
		# First, decide no then push to stack.
		new_frame = deepcopy(frame)
		new_frame.next = next-1
		stack.append(new_frame)

		# Then, decide yes and keep processing
		frame.gens.add(next)
		for c in ds[next][next]:
			frame.span.add(c)
		for other in frame.span:
			for c in ds[next][other]:
				frame.span.add(c)
	# No more decisions, so this is a minimal generating set.
	out.append(FrozenBitset(frame.gens))

print(len(out))
