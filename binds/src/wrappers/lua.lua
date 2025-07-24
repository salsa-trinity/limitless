-- import ffi
local ffi = require("ffi")
ffi.cdef([[
    const char* __limless_lib_logm_ffi(int);
    void __limless_fake(void*);
]])
-- TODO: remove os specific file extension
local clib = ffi.load("./target/debug/libffi.dylib")

-- define lua wrapper
local engine = {}
engine.logm = function(a)
	if type(a) ~= "number" then
		error("Parameter 'a', must be a number.")
	end
	clib.__limless_lib_logm_ffi(a)
end
-- NOTE: non existant functions dont cause problems in the
-- lua wrapper unless called, I should change the ffi.rs function
-- definictions depending on the `features`, for them to just ask
-- the user to import the feature and rebuild
engine.fake = function()
	clib.__limless_fake()
end

return engine
