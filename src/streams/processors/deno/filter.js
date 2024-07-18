let events = Deno.core.ops.get_events();

events.map((event) => {
	event.payload = {"msg": "Deno touches your stuff"};
	return event;
}).slice(1);
