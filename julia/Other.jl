module Other

export get_input, check_abs_diff

# TODO: argument to allow try once and fail
function get_input(t::Type{T}, prompt::String="")::T where {T}
    print(prompt)
    while true
        line = strip(readline())
        try return parse(t, line)
        catch
        end
    end
end

end
