export declare function print_name_wasm(): void
export declare function print_wasm(s: i32): void
export declare function get_transform(): i32
export declare function print_transform(s: i32): void

export class Transform {
  pos: Vec3;
  rotation: Quat;
  constructor(pos: Vec3, rot: Quat) {
    this.pos = pos;
    this.rotation = rot;
  }
  public static from_ptr(ptr: isize): Transform {
    let t = Transform.from_ptr_inner(ptr);
    heap.free(ptr);
    return t;
  }
  public static from_ptr_inner(ptr: isize): Transform {
    let vec = Vec3.from_ptr_inner(ptr);
    let quat = Quat.from_ptr_inner(ptr + (sizeof<f32>() * 3));
    return new Transform(vec, quat);
  }
  public static size(): u32 {
    return Vec3.size() + Quat.size();
  }
  public into_ptr(): u32 {
    let ptr = heap.alloc(Transform.size())
    this.write_to_ptr(ptr);
    return ptr;
  }
  public write_to_ptr(ptr: u32): u32 {
    let mut_ptr = this.pos.write_to_ptr(ptr);
    mut_ptr = this.rotation.write_to_ptr(mut_ptr)
    return mut_ptr;
  }
}

export class Vec3 {
  x: f32;
  y: f32;
  z: f32;
  constructor(x: f32, y: f32, z: f32) {
    this.x = x;
    this.y = y;
    this.z = z;
  }
  public static from_ptr(ptr: isize): Vec3 {
    let v = Vec3.from_ptr_inner(ptr);
    heap.free(ptr);
    return v;
  }
  public static from_ptr_inner(ptr: isize): Vec3 {
    let x = load<f32>(ptr);
    let y = load<f32>(ptr + sizeof<f32>());
    let z = load<f32>(ptr + (sizeof<f32>() * 2));
    return Vec3.new(x, y, z);
  }
  public static new(x: f32, y: f32, z: f32): Vec3 {
    return new Vec3(x, y, z);
  }
  public static size(): u32 {
    return sizeof<f32>() * 3;
  }
  public write_to_ptr(ptr: u32): u32 {
    store<f32>(ptr, this.x);
    store<f32>(ptr + sizeof<f32>(), this.y);
    store<f32>(ptr + (sizeof<f32>() * 2), this.z);
    return ptr + Vec3.size();
  }
}

export class Quat {
  x: f32;
  y: f32;
  z: f32;
  w: f32;
  constructor(x: f32, y: f32, z: f32, w: f32) {
    this.x = x;
    this.y = y;
    this.z = z;
    this.w = w;
  }
  public static new(x: f32, y: f32, z: f32, w: f32): Quat {
    return new Quat(x, y, z, w);
  }
  public static from_ptr(ptr: isize): Quat {
    let q = Quat.from_ptr_inner(ptr);
    heap.free(ptr);
    return q;
  }
  public static from_ptr_inner(ptr: isize): Quat {

    let x = load<f32>(ptr);
    let y = load<f32>(ptr + sizeof<f32>());
    let z = load<f32>(ptr + (sizeof<f32>() * 2));
    let w = load<f32>(ptr + (sizeof<f32>() * 3));
    return Quat.new(x, y, z, w);
  }
  public static size(): u32 {
    return sizeof<f32>() * 3;
  }
  public write_to_ptr(ptr: u32): u32 {
    store<f32>(ptr, this.x);
    store<f32>(ptr + sizeof<f32>(), this.y);
    store<f32>(ptr + (sizeof<f32>() * 2), this.z);
    store<f32>(ptr + (sizeof<f32>() * 3), this.w);
    return ptr + Quat.size();
  }
}

export function alloc(size: u32): u32 {
  return heap.alloc(size);
}

export function main(): void {
  print_name_wasm();
  let transform_ptr = get_transform();



  let transform = Transform.from_ptr(transform_ptr);
  transform.pos.z = -420;
  let ptr = transform.into_ptr();
  print_transform(ptr);
  heap.free(ptr);

  let pos_string = `x: ${transform.pos.x}, y: ${transform.pos.y}, z: ${transform.pos.z}`
  let rot_string = `x: ${transform.rotation.x}, y: ${transform.rotation.y}, z: ${transform.rotation.z}, w: ${transform.rotation.w}`
  let pos_str = String.UTF8.encode(pos_string, true);
  let rot_str = String.UTF8.encode(rot_string, true);
  print_wasm(changetype<usize>(pos_str));
  print_wasm(changetype<usize>(rot_str));
}

export function panic(message: string, fileName: string, line: u32, column: u32): void {
  print_name_wasm();
  // to lazy to deref the message/filename pointers rn
  let str = `Panic! ${fileName}:${line}:${column} ${message}`;
  let the_cooler_str = String.UTF8.encode(str, true);

  print_wasm(changetype<usize>(the_cooler_str));
} 
