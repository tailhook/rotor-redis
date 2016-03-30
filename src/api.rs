use {Redis, Context};

use rotor_stream::ActiveStream;

//struct Cmd { }

impl<C, S> Redis<C, S>
    where C: Context, S: ActiveStream
{

}
