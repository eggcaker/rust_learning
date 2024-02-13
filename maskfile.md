## services

> Commands related to starting and stopping services

### services start (service_name)

> Start a service.

~~~bash
echo "Starting service $service_name"
~~~

### services stop (service_name)

> Stop a service.

~~~nu
[[a b c]; [1 2 3] [ 4 5 6]] |get a 
~~~



## shell 
> An example shell 
Valid lang codes: sh 

~~~nu 
ls ~/ |get name 
~~~

## node
> An example node script 

~~~js
const {name} = process.env; 
console.log(`Hello, ${name}!`);
~~~