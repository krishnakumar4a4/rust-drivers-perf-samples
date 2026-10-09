mod shard_00;
mod shard_01;
mod shard_02;
mod shard_03;
mod shard_04;
mod shard_05;
mod shard_06;
mod shard_07;
mod shard_08;
mod shard_09;

pub(crate) fn emit() {
    shard_00::emit();
    shard_01::emit();
    shard_02::emit();
    shard_03::emit();
    shard_04::emit();
    shard_05::emit();
    shard_06::emit();
    shard_07::emit();
    shard_08::emit();
    shard_09::emit();
}
