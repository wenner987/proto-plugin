// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: student.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_student_2eproto
#define GOOGLE_PROTOBUF_INCLUDED_student_2eproto

#include <limits>
#include <string>

#include <google/protobuf/port_def.inc>
#if PROTOBUF_VERSION < 3015000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers. Please update
#error your headers.
#endif
#if 3015008 < PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers. Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/port_undef.inc>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_table_driven.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/metadata_lite.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/unknown_field_set.h>
#include <google/protobuf/descriptor.pb.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>
#define PROTOBUF_INTERNAL_EXPORT_student_2eproto
PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_student_2eproto {
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTableField entries[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::AuxiliaryParseTableField aux[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTable schema[1]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::FieldMetadata field_metadata[];
  static const ::PROTOBUF_NAMESPACE_ID::internal::SerializationTable serialization_table[];
  static const ::PROTOBUF_NAMESPACE_ID::uint32 offsets[];
};
extern const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable descriptor_table_student_2eproto;
::PROTOBUF_NAMESPACE_ID::Metadata descriptor_table_student_2eproto_metadata_getter(int index);
namespace com {
namespace test {
class Student;
struct StudentDefaultTypeInternal;
extern StudentDefaultTypeInternal _Student_default_instance_;
}  // namespace test
}  // namespace com
PROTOBUF_NAMESPACE_OPEN
template<> ::com::test::Student* Arena::CreateMaybeMessage<::com::test::Student>(Arena*);
PROTOBUF_NAMESPACE_CLOSE
namespace com {
namespace test {

// ===================================================================

class Student PROTOBUF_FINAL :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:com.test.Student) */ {
 public:
  inline Student() : Student(nullptr) {}
  virtual ~Student();
  explicit constexpr Student(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  Student(const Student& from);
  Student(Student&& from) noexcept
    : Student() {
    *this = ::std::move(from);
  }

  inline Student& operator=(const Student& from) {
    CopyFrom(from);
    return *this;
  }
  inline Student& operator=(Student&& from) noexcept {
    if (GetArena() == from.GetArena()) {
      if (this != &from) InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return GetMetadataStatic().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return GetMetadataStatic().reflection;
  }
  static const Student& default_instance() {
    return *internal_default_instance();
  }
  static inline const Student* internal_default_instance() {
    return reinterpret_cast<const Student*>(
               &_Student_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(Student& a, Student& b) {
    a.Swap(&b);
  }
  inline void Swap(Student* other) {
    if (other == this) return;
    if (GetArena() == other->GetArena()) {
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(Student* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetArena() == other->GetArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  inline Student* New() const final {
    return CreateMaybeMessage<Student>(nullptr);
  }

  Student* New(::PROTOBUF_NAMESPACE_ID::Arena* arena) const final {
    return CreateMaybeMessage<Student>(arena);
  }
  void CopyFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void MergeFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void CopyFrom(const Student& from);
  void MergeFrom(const Student& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  ::PROTOBUF_NAMESPACE_ID::uint8* _InternalSerialize(
      ::PROTOBUF_NAMESPACE_ID::uint8* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _cached_size_.Get(); }

  private:
  inline void SharedCtor();
  inline void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(Student* other);
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "com.test.Student";
  }
  protected:
  explicit Student(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  private:
  static void ArenaDtor(void* object);
  inline void RegisterArenaDtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;
  private:
  static ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadataStatic() {
    return ::descriptor_table_student_2eproto_metadata_getter(kIndexInFileMessages);
  }

  public:

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kNameFieldNumber = 1,
    kAgeFieldNumber = 2,
    kScoreFieldNumber = 3,
  };
  // string name = 1 [(.com.test.not_null) = true];
  void clear_name();
  const std::string& name() const;
  void set_name(const std::string& value);
  void set_name(std::string&& value);
  void set_name(const char* value);
  void set_name(const char* value, size_t size);
  std::string* mutable_name();
  std::string* release_name();
  void set_allocated_name(std::string* name);
  private:
  const std::string& _internal_name() const;
  void _internal_set_name(const std::string& value);
  std::string* _internal_mutable_name();
  public:

  // int32 age = 2 [(.com.test.not_null) = true];
  void clear_age();
  ::PROTOBUF_NAMESPACE_ID::int32 age() const;
  void set_age(::PROTOBUF_NAMESPACE_ID::int32 value);
  private:
  ::PROTOBUF_NAMESPACE_ID::int32 _internal_age() const;
  void _internal_set_age(::PROTOBUF_NAMESPACE_ID::int32 value);
  public:

  // int32 score = 3;
  void clear_score();
  ::PROTOBUF_NAMESPACE_ID::int32 score() const;
  void set_score(::PROTOBUF_NAMESPACE_ID::int32 value);
  private:
  ::PROTOBUF_NAMESPACE_ID::int32 _internal_score() const;
  void _internal_set_score(::PROTOBUF_NAMESPACE_ID::int32 value);
  public:

  // @@protoc_insertion_point(class_scope:com.test.Student)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr name_;
  ::PROTOBUF_NAMESPACE_ID::int32 age_;
  ::PROTOBUF_NAMESPACE_ID::int32 score_;
  mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  friend struct ::TableStruct_student_2eproto;
};
// ===================================================================

static const int kCsvFileNameFieldNumber = 50000;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::MessageOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::StringTypeTraits, 9, false >
  csv_file_name;
static const int kMarkExtendFieldNumber = 50001;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::MessageOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< bool >, 8, false >
  mark_extend;
static const int kIndexFieldNumber = 50002;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::MessageOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::StringTypeTraits, 9, false >
  index;
static const int kNotNullFieldNumber = 51001;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< bool >, 8, false >
  not_null;
static const int kLtFieldNumber = 51002;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< ::PROTOBUF_NAMESPACE_ID::int32 >, 5, false >
  lt;
static const int kGtFieldNumber = 51003;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< ::PROTOBUF_NAMESPACE_ID::int32 >, 5, false >
  gt;
static const int kLteFieldNumber = 51004;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< ::PROTOBUF_NAMESPACE_ID::int32 >, 5, false >
  lte;
static const int kGteFieldNumber = 51005;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::PrimitiveTypeTraits< ::PROTOBUF_NAMESPACE_ID::int32 >, 5, false >
  gte;
static const int kSplitorFieldNumber = 51006;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::StringTypeTraits, 9, false >
  splitor;
static const int kInRangeFieldNumber = 51007;
extern ::PROTOBUF_NAMESPACE_ID::internal::ExtensionIdentifier< ::google::protobuf::FieldOptions,
    ::PROTOBUF_NAMESPACE_ID::internal::StringTypeTraits, 9, false >
  in_range;

// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// Student

// string name = 1 [(.com.test.not_null) = true];
inline void Student::clear_name() {
  name_.ClearToEmpty();
}
inline const std::string& Student::name() const {
  // @@protoc_insertion_point(field_get:com.test.Student.name)
  return _internal_name();
}
inline void Student::set_name(const std::string& value) {
  _internal_set_name(value);
  // @@protoc_insertion_point(field_set:com.test.Student.name)
}
inline std::string* Student::mutable_name() {
  // @@protoc_insertion_point(field_mutable:com.test.Student.name)
  return _internal_mutable_name();
}
inline const std::string& Student::_internal_name() const {
  return name_.Get();
}
inline void Student::_internal_set_name(const std::string& value) {
  
  name_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, value, GetArena());
}
inline void Student::set_name(std::string&& value) {
  
  name_.Set(
    ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, ::std::move(value), GetArena());
  // @@protoc_insertion_point(field_set_rvalue:com.test.Student.name)
}
inline void Student::set_name(const char* value) {
  GOOGLE_DCHECK(value != nullptr);
  
  name_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, ::std::string(value), GetArena());
  // @@protoc_insertion_point(field_set_char:com.test.Student.name)
}
inline void Student::set_name(const char* value,
    size_t size) {
  
  name_.Set(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, ::std::string(
      reinterpret_cast<const char*>(value), size), GetArena());
  // @@protoc_insertion_point(field_set_pointer:com.test.Student.name)
}
inline std::string* Student::_internal_mutable_name() {
  
  return name_.Mutable(::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr::EmptyDefault{}, GetArena());
}
inline std::string* Student::release_name() {
  // @@protoc_insertion_point(field_release:com.test.Student.name)
  return name_.Release(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline void Student::set_allocated_name(std::string* name) {
  if (name != nullptr) {
    
  } else {
    
  }
  name_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), name,
      GetArena());
  // @@protoc_insertion_point(field_set_allocated:com.test.Student.name)
}

// int32 age = 2 [(.com.test.not_null) = true];
inline void Student::clear_age() {
  age_ = 0;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Student::_internal_age() const {
  return age_;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Student::age() const {
  // @@protoc_insertion_point(field_get:com.test.Student.age)
  return _internal_age();
}
inline void Student::_internal_set_age(::PROTOBUF_NAMESPACE_ID::int32 value) {
  
  age_ = value;
}
inline void Student::set_age(::PROTOBUF_NAMESPACE_ID::int32 value) {
  _internal_set_age(value);
  // @@protoc_insertion_point(field_set:com.test.Student.age)
}

// int32 score = 3;
inline void Student::clear_score() {
  score_ = 0;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Student::_internal_score() const {
  return score_;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Student::score() const {
  // @@protoc_insertion_point(field_get:com.test.Student.score)
  return _internal_score();
}
inline void Student::_internal_set_score(::PROTOBUF_NAMESPACE_ID::int32 value) {
  
  score_ = value;
}
inline void Student::set_score(::PROTOBUF_NAMESPACE_ID::int32 value) {
  _internal_set_score(value);
  // @@protoc_insertion_point(field_set:com.test.Student.score)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)

}  // namespace test
}  // namespace com

// @@protoc_insertion_point(global_scope)

#include <google/protobuf/port_undef.inc>
#endif  // GOOGLE_PROTOBUF_INCLUDED_GOOGLE_PROTOBUF_INCLUDED_student_2eproto
