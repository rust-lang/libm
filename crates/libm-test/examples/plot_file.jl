"A quick script to for plotting a list of floats.

Takes a list of floats and a real function, plots both on a graph in both
linear and log scale. Requires [Makie] (specifically CairoMakie) for plotting.

[Makie]: https://docs.makie.org/stable/
"

using CairoMakie

CairoMakie.activate!(px_per_unit=10)

"Apply a function, returning the default if there is a domain error"
function map_or_default(
    input::AbstractFloat,
    f::Function,
    default::AbstractFloat
)::AbstractFloat
    try
        return f(input)
    catch
        return default
    end
end

"Read inputs from a file, create both linear and log plots"
function plot_one(
    fig::Figure,
    base_name::String,
    fn_name::String,
    f::Function;
    xlims::Union{Tuple{Any,Any},Nothing},
    xlims_log::Union{Tuple{Any,Any},Nothing},
)::Nothing
    float_file = "$base_name.txt"
    lin_out_file = "$base_name.png"
    log_out_file = "$base_name-log.png"

    if xlims === nothing
        xlims = (-6, 6)
    end
    if xlims_log === nothing
        xlims_log = (xlims[1] * 500, xlims[2] * 500)
    end

    inputs = readlines(float_file)

    # Parse floats
    x = map((v) -> parse(Float32, v), inputs)
    # Apply function to the test points
    y = map((v) -> map_or_default(v, f, 0.0), x)

    # Plot the scatter plot of our checked values as well as the continuous function
    ax = Axis(fig[1, 1], limits=(xlims, nothing), title=fn_name)
    lines!(ax, xlims[1] .. xlims[2], f, color=(:blue, 0.4))
    scatter!(ax, x, y, color=(:green, 0.3))
    save(lin_out_file, fig)
    delete!(ax)

    # Same as above on a log scale
    ax = Axis(
        fig[1, 1],
        limits=(xlims_log, nothing),
        xscale=Makie.pseudolog10,
        title="$fn_name (log scale)"
    )
    lines!(ax, xlims_log[1] .. xlims_log[2], f, color=(:blue, 0.4))
    scatter!(ax, x, y, color=(:green, 0.3))
    save(log_out_file, fig)
    delete!(ax)
end

# Args alternate `name1 path1 name2 path2`
fn_names = ARGS[1:2:end]
base_names = ARGS[2:2:end]

for idx in eachindex(fn_names)
    fn_name = fn_names[idx]
    base_name = base_names[idx]

    xlims = nothing
    xlims_log = nothing

    fig = Figure()

    # Map string function names to callable functions
    if fn_name == "cos"
        f = cos
        xlims_log = (-pi * 10, pi * 10)
    elseif fn_name == "cbrt"
        f = cbrt
        xlims = (-2.0, 2.0)
    elseif fn_name == "sqrt"
        f = sqrt
        xlims = (-1.1, 6.0)
        xlims_log = (-1.1, 5000.0)
    else
        println("unrecognized function name `$fn_name`; update plot_file.jl")
    end

    println("plotting $fn_name")
    plot_one(
        fig,
        base_name,
        fn_name,
        f,
        xlims=xlims,
        xlims_log=xlims_log,
    )
end

base_name = ARGS[1]
fn_name = ARGS[2]
