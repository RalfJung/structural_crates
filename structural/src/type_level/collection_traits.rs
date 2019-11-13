mod push_back;
mod tuple_impls;

/// A trait which allows getting a heterogeneous collection type (like tuples),
/// in which `Type` was added after the last element.
pub trait PushBack_<Type>{
    type Output;
}

/// This allows getting a heterogeneous collection type (like tuples),
/// in which `Type` was added after the last element.
pub type PushBack<This,Type>=
    <This as PushBack_<Type>>::Output;


/// Gets the `TList` equivalent of `Self`.
pub trait ToTList_{
    type Output;
}


/// Gets the `TList` type equivalent of `This`.
pub type ToTList<This>=
    <This as ToTList_>::Output;



/// Gets a tuple type equivalent of `Self`.
pub trait ToTuple_{
    type Output;
}

/// Gets a tuple type equivalent of `This`.
pub type ToTuple<This>=
    <This as ToTuple_>::Output;


/// Gets this collection type with `Other` appended at the end.
pub trait Append_<Other>{
    type Output;
}


/// Gets the `This` collection type with `Other` appended at the end.
pub type Append<This,Other>=
    <This as Append_<Other>>::Output;

