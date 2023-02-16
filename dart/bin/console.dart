import 'package:console/console.dart' as console;
import 'package:messagepack/messagepack.dart';
import 'dart:typed_data'; //Bundled with Dart

void simple() {
  final p = Packer();
  p.packInt(1);
  p.packInt(2);
  final bytes = p.takeBytes(); //Uint8List
  print(bytes);

  final u = Unpacker(bytes);
  final n1 = u.unpackInt();
  final n2 = u.unpackInt();
  print('unpacked n1=$n1 n2=$n2');
}

void differentTypesSimple() {
  final p = Packer();
  p.packInt(1);
  p.packBool(true);
  final bytes = p.takeBytes(); //Uint8List
  print(bytes);

  final u = Unpacker(bytes);
  print(u.unpackInt());
  print(u.unpackBool());
}

void iterableAndMap() {
  final list = ['i1', 'i2'];
  final map = {'k1': 11, 'k2': 22};
  final p = Packer();
  p.packListLength(list.length);
  list.forEach(p.packString);
  p.packMapLength(map.length);
  map.forEach((key, v) {
    p.packString(key);
    p.packInt(v);
  });
  final bytes = p.takeBytes();

  final u = Unpacker(bytes);
  final listLength = u.unpackListLength();
  for (int i = 0; i < listLength; i++) {
    print(u.unpackString());
  }
  final mapLength = u.unpackMapLength();
  for (int i = 0; i < mapLength; i++) {
    print(u.unpackString());
    print(u.unpackInt());
  }
}

void differentTypesComplex() {
  final p = Packer()
    ..packInt(99)
    ..packBool(true)
    ..packString('hi')
    ..packNull()
    ..packString(null)
    ..packBinary(<int>[104, 105]) // hi codes
    ..packListLength(2) // pack 2 elements list ['elem1',3.14]
    ..packString('elem1')
    ..packDouble(3.14)
    ..packString('continue to pack other elements')
    ..packMapLength(2) //map {'key1':false, 'key2',3.14}
    ..packString('key1') //pack key1
    ..packBool(false) //pack value1
    ..packString('key12') //pack key1
    ..packDouble(3.13); //pack value1

  final bytes = p.takeBytes();
  final u = Unpacker(bytes);
  //Unpack the same sequential/streaming way
  u.unpackInt();
  //....
}

void main(List<String> arguments) {
  print('Hello world  ...: ${console.calculate()}!');
  simple();
  differentTypesComplex();
  final p = Packer()
    ..packListLength(10) //pack 10 different types items to list
    ..packInt(99)
    ..packBool(true)
    ..packString('hi')
    ..packNull() // explicitly pack null
    ..packString(null) // implicitly any type can accept null
    ..packBinary(<int>[104, 105]) // hi codes
    ..packListLength(2) // pack 2 elements list ['elem1',3.14]
    ..packString('elem1') // list[0]
    ..packDouble(3.14) // list[1]
    ..packString('continue to pack other elements')
    ..packMapLength(1) // map {'key1':false}
    ..packString('key1') // map key
    ..packBool(false) // map value
    ..packInt(9223372036854775807); // next root list item (map ended)

  //final bytes = p.takeBytes();
  final bytes = Uint8List.fromList([
    149,
    167,
    67,
    108,
    105,
    109,
    97,
    116,
    101,
    5,
    171,
    84,
    101,
    109,
    112,
    101,
    114,
    97,
    116,
    117,
    114,
    101,
    202,
    64,
    95,
    124,
    238,
    205,
    3,
    22,
  ]);
  print(bytes);
  final u = Unpacker(bytes);
  // Unpack by the same sequential/streaming way
  // or implicitly/automatically
  final l = u.unpackList(); //List<Object>
  print(l);
  // [99, true, hi, null, null, [104, 105], [elem1, 3.14], continue to pack other elements, {key1: false}, 9223372036854775807]
}
