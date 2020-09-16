### A Pluto.jl notebook ###
# v0.11.14

using Markdown
using InteractiveUtils

# ╔═╡ 5dc3b4c6-f4b9-11ea-3245-6d726e7e013f
begin
	using Images
	philip = load("philip.jpg")
end

# ╔═╡ f9893e7c-f4b8-11ea-22ab-7b9845fbe9f3
url = "https://i.imgur.com/VGPeJ6s.jpg"

# ╔═╡ 4e582dbe-f4b9-11ea-29c6-8d62ded32a99
download(url, "philip.jpg")

# ╔═╡ a28945a2-f4ba-11ea-1ae0-abcdbc83b171
typeof(philip)

# ╔═╡ cffcfb96-f4ba-11ea-1cd7-35a30f8077a7
RGBX(0.4, 0.6, 0.6)

# ╔═╡ e59de58c-f4ba-11ea-157e-096eed92ecfb
size(philip)

# ╔═╡ f35c83fe-f4ba-11ea-08c1-7515d7146923
philip[1:1000,1:400]

# ╔═╡ 1c3d9b0c-f4bb-11ea-1638-3d2076c23093
begin
	(h, w) = size(philip)
	head = philip[(h ÷ 2):h, (w ÷ 10): (9w ÷ 10)]
end

# ╔═╡ 567b6ffe-f4bb-11ea-35c7-d5e123d6cb40
size(head)

# ╔═╡ 644b4988-f4bb-11ea-1d5d-73900abda3d2
typeof([head head])

# ╔═╡ 7745ad30-f4bb-11ea-107d-b16cb31cfcff
[
	head 				  reverse(head, dims=2)
	reverse(head, dims=1) reverse(reverse(head,dims=1), dims=2)
]

# ╔═╡ d1f9c798-f4bb-11ea-0b58-09ef4a2ad157
new_phil = copy(head)

# ╔═╡ e7d2f878-f4bb-11ea-20be-91a92d4b1174
red = RGB(1,0,0)

# ╔═╡ f0b55c9c-f4bb-11ea-0300-c9fae6038d52
for i in 1:100
	for j in 1:300
		new_phil[i,j] = red
	end
end


# ╔═╡ 083eb4d0-f4bc-11ea-23c3-53c7e93e3d2c
new_phil

# ╔═╡ 0aaffd78-f4bc-11ea-0c9d-07264a6d052b
begin 
	new_phil2 = copy(new_phil)
	new_phil2[100:200, 1:100] .= RGB(0,1,0)
	new_phil2
end

# ╔═╡ 3a6bf530-f4bc-11ea-391e-8772d38656c0
function redify(color)
	return RGB(color.r, 0, 0)
end

# ╔═╡ 49c8e54c-f4bc-11ea-0ce9-7fa4ffd7d818
begin
	color = RGB(0.5, 0.5, 0.2)
	
	[color, redify(color)]
end

# ╔═╡ 5cef3af4-f4bc-11ea-1d40-cde5ec741881
redify.(philip)

# ╔═╡ 674acdf6-f4bc-11ea-1bfd-9fbfa76c82c5
begin
	poor_phil = decimate(new_phil, 5)
	size(poor_phil)
end

# ╔═╡ 02fcc358-f4bd-11ea-300f-d77c789739fc
convolve(poor_phil, blur(2))

# ╔═╡ Cell order:
# ╠═f9893e7c-f4b8-11ea-22ab-7b9845fbe9f3
# ╠═4e582dbe-f4b9-11ea-29c6-8d62ded32a99
# ╠═5dc3b4c6-f4b9-11ea-3245-6d726e7e013f
# ╠═a28945a2-f4ba-11ea-1ae0-abcdbc83b171
# ╠═cffcfb96-f4ba-11ea-1cd7-35a30f8077a7
# ╠═e59de58c-f4ba-11ea-157e-096eed92ecfb
# ╠═f35c83fe-f4ba-11ea-08c1-7515d7146923
# ╠═1c3d9b0c-f4bb-11ea-1638-3d2076c23093
# ╠═567b6ffe-f4bb-11ea-35c7-d5e123d6cb40
# ╠═644b4988-f4bb-11ea-1d5d-73900abda3d2
# ╠═7745ad30-f4bb-11ea-107d-b16cb31cfcff
# ╠═d1f9c798-f4bb-11ea-0b58-09ef4a2ad157
# ╠═e7d2f878-f4bb-11ea-20be-91a92d4b1174
# ╠═f0b55c9c-f4bb-11ea-0300-c9fae6038d52
# ╠═083eb4d0-f4bc-11ea-23c3-53c7e93e3d2c
# ╠═0aaffd78-f4bc-11ea-0c9d-07264a6d052b
# ╠═3a6bf530-f4bc-11ea-391e-8772d38656c0
# ╠═49c8e54c-f4bc-11ea-0ce9-7fa4ffd7d818
# ╠═5cef3af4-f4bc-11ea-1d40-cde5ec741881
# ╠═674acdf6-f4bc-11ea-1bfd-9fbfa76c82c5
# ╠═02fcc358-f4bd-11ea-300f-d77c789739fc
