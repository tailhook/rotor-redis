(function() {var implementors = {};
implementors['void'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='void/enum.Void.html' title='void::Void'>Void</a>",];implementors['log'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='log/struct.SetLoggerError.html' title='log::SetLoggerError'>SetLoggerError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='log/struct.ShutdownLoggerError.html' title='log::ShutdownLoggerError'>ShutdownLoggerError</a>",];implementors['time'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='time/struct.OutOfRangeError.html' title='time::OutOfRangeError'>OutOfRangeError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='time/enum.ParseError.html' title='time::ParseError'>ParseError</a>",];implementors['nix'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='nix/enum.Error.html' title='nix::Error'>Error</a>",];implementors['mio'] = ["impl&lt;M: <a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='mio/enum.NotifyError.html' title='mio::NotifyError'>NotifyError</a>&lt;M&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='nix/enum.Error.html' title='nix::Error'>Error</a>",];implementors['rotor'] = ["impl&lt;M&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='rotor/mio/enum.NotifyError.html' title='rotor::mio::NotifyError'>NotifyError</a>&lt;M&gt; <span class='where'>where M: <a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='rotor/void/enum.Void.html' title='rotor::void::Void'>Void</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='rotor/enum.WakeupError.html' title='rotor::WakeupError'>WakeupError</a>","impl&lt;S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='rotor/enum.SpawnError.html' title='rotor::SpawnError'>SpawnError</a>&lt;S&gt;",];implementors['rotor_stream'] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='enum' href='rotor_stream/enum.Exception.html' title='rotor_stream::Exception'>Exception</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='rotor_stream/struct.ProtocolStop.html' title='rotor_stream::ProtocolStop'>ProtocolStop</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
