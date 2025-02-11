
using Printf
using Remez

function main()
    run_one("f64", "hf64!", 53)
end

function run_one(name::String, hf::String, precision::Integer)
    setprecision(precision)

    println("Constants for ", name)
    
    println("const ESCALE: [Self; 3] = [")
    for n in 0:2
        val = big(2) ^ (n / 3)
        @printf "    %s(\"%a\"),\n" hf val
    end
    print("];")

end

main()
