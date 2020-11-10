require 'ffi'

module Twist
  extend FFI::Library
  lib_name = "libtwist_rs.#{::FFI::Platform::LIBSUFFIX}"

  ffi_lib "#{File.expand_path(File.dirname(__FILE__))}/../../target/release/#{lib_name}"

  attach_function :search, [:string, :string], :string
  attach_function :string_free, [:string], :void
end

result = Twist.search(ENV["auth"] || "", "pothix")
puts result
Twist.string_free(result)
