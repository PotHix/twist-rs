require 'ffi'

module Twist
  extend FFI::Library
  ffi_lib "#{File.expand_path(File.dirname(__FILE__))}/../../target/debug/libtwist_rs.so"
  attach_function :search, [:string, :string], :string
end

puts Twist.search(ENV["auth"] || "", "pothix")
