use bevy::tasks::Scope as BevyScope;
use wacky_bag::traits::scope_no_ret::{ThreadScope, ThreadScopeCreator, ThreadScopeUser};

pub struct ComputeTaskPoolScope<'scope, 'env: 'scope>(&'scope BevyScope<'scope, 'env, ()>);

impl<'scope,'env:'scope> ThreadScope<'scope>
	for ComputeTaskPoolScope<'scope, 'env>
{

	fn spawn<F>(&'scope self, f: F) -> ()
	where
		F: FnOnce() -> () + std::marker::Send + 'scope,
	{
		self.0.spawn(async move { f() });
	}
	
}
// impl<'scope, 'env: 'scope, ScopeFuncOutput> wacky_bag::traits::scope::ThreadScope<'scope, ScopeFuncOutput>
//     for ComputeTaskPoolScope<'scope, 'env, ScopeFuncOutput>
// where
//     ScopeFuncOutput: Send,
// {
//     fn spawn<F>(&'scope self, f: F) -> ()
//     where
//         F: FnOnce() -> ScopeFuncOutput + std::marker::Send + 'scope,
//     {
//         self.0.spawn(async move { f() });
//     }
// }


pub struct ComputeTaskPoolScopeCreator;

impl ThreadScopeCreator for ComputeTaskPoolScopeCreator {
    fn scope<'env,F>(&self,f:F ) -> ()
        where F: ThreadScopeUser<'env>,
    {
        bevy::tasks::ComputeTaskPool::get().scope(move |s:&BevyScope<'_,'env, _ >|{
            let ntscope=ComputeTaskPoolScope(s);
            // bbevy::tasks::TaskPool::scope did same
            let extended_ntscope: &'env ComputeTaskPoolScope<'_, 'env> =
                unsafe { std::mem::transmute(&ntscope) };
            f.use_scope(extended_ntscope);
        });
    }
}