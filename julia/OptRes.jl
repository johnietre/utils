module OptRes

export Option, Result,
    takevalue!, takeerr!,
    issome, isnone,
    hasvalue, haserr,
    isok, iserr, ispartial

mutable struct Option{T}
    value::Union{T, Nothing}
end

Option{T}() where {T} = Option{T}(nothing)

#=
function Option(value::Union{T, Nothing}=nothing)::Option{T} where T
    Option(value)
end
=#

function takevalue!(opt::Option{T})::Union{T, Nothing} where {T}
    val = opt.value
    opt.value = nothing
    val
end

function issome(opt::Option{T})::Bool where {T}
    isnothing(opt.value)
end

function isnone(opt::Option{T})::Bool where {T}
    isnothing(opt.value)
end

# TODO
function Base.iterate(opt::Option{T})::Union{Tuple{T, Int64}, Nothing} where {T}
    val = takevalue!(opt)
    if isnothing(val)
        nothing
    else
        (val, 2)
    end
end

function Base.show(io::IO, opt::Option{T}) where {T}
    if issome(opt)
        write(io, "Some($(opt.value))")
    else
        write(io, "None")
    end
end

@kwdef mutable struct Result{T, E}
    value::Union{T, Nothing}
    err::Union{E, Nothing}
end

function takevalue!(res::Result{T, E})::Union{T, Nothing} where {T, E}
    if isok(res)
        val = res.value
        res.value = nothing
        val
    else
        nothing
    end
end

function takeerr!(res::Result{T, E})::Union{E, Nothing} where {T, E}
    if isok(res)
        err = res.err
        res.err = nothing
        err
    else
        nothing
    end
end

function hasvalue(res::Result{T, E})::Bool where {T, E}
    !isnothing(res.value)
end

function haserr(res::Result{T, E})::Bool where {T, E}
    !isnothing(res.err)
end

function isok(res::Result{T, E})::Bool where {T, E}
    hasvalue(res) && !haserr(res)
end

function iserr(res::Result{T, E})::Bool where {T, E}
    !hasvalue(res) && haserr(res)
end

function ispartial(res::Result{T, E})::Bool where {T, E}
    hasvalue(res) && haserr(res)
end

function isempty(res::Result{T, E})::Bool where {T, E}
    !hasvalue(res) && !haserr(res)
end

function Base.show(io::IO, res::Result{T, E}) where {T, E}
    if isok(res)
        write(io, "Ok($(res.value))")
    elseif iserr(res)
        write(io, "Err($(res.err))")
    elseif ispartial(res)
        write(io, "Partial(Value=$(res.value) | Err=$(res.err))")
    else
        write(io, "Empty")
    end
end

end
