-- Unpacks an .map and .mpak for loading, calls a function on it, then writes it back when complete.
local function edit_mpak(dat, name, cb)
    local mpak = chaudloader.Mpak(
        dat:read_file(name .. ".map"),
        dat:read_file(name .. ".mpak")
    )
    cb(mpak)
    local raw_map, raw_mpak = mpak:pack()
    dat:write_file(name .. ".map", raw_map)
    dat:write_file(name .. ".mpak", raw_mpak)
end

-- Reads a file as a ByteArray and saves it back when done.
local function edit_as_bytearray(dat, path, cb)
    local ba = chaudloader.ByteArray(dat:read_file(path))
    cb(ba)
    dat:write_file(path, ba:pack())
end

-- Unpacks msg data, calls a function on it, then writes it back when complete.
local function edit_msg(mpak, address, cb)
    mpak[address] = chaudloader.pack_msg(cb(chaudloader.unpack_msg(mpak[address])))
end

-- Merges two messages together, preferring the latter one.
--
-- Only non-empty entries from the new msg data will be merged.
local function merge_msg(old, new)
    for i, entry in ipairs(new) do
        if entry ~= "" then
            old[i] = entry
        else
        end
    end
    return old
end

-- Merges all msgs from a directory.
--
-- The directory must contain files named addresses of msgs to replace, followed by `.msg`.
--
-- The addresses may be either mapped ROM addresses (08XXXXXX) or unmapped file offsets (00XXXXXX): if they are unmapped file offsets, they will be automatically transformed into mapped ROM addresses.
local function merge_msgs_from_mod_directory(mpak, dir)
    for _, filename in ipairs(chaudloader.list_mod_directory(dir)) do
        local raw_addr = string.match(filename, "^(%x+).msg$")
        if raw_addr == nil then
            goto continue
        end
        local addr = tonumber(raw_addr, 16) | 0x08000000
        edit_msg(mpak, addr, function (msg)
            return merge_msg(msg, chaudloader.unpack_msg(chaudloader.read_mod_file(dir .. '/' .. filename)))
        end)
        ::continue::
    end
end

return {
    edit_mpak = edit_mpak,
    edit_msg = edit_msg,
    edit_as_bytearray = edit_as_bytearray,
    merge_msg = merge_msg,
    merge_msgs_from_mod_directory = merge_msgs_from_mod_directory,
}
