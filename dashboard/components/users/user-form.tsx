"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { createUserSchema, updateUserSchema, type CreateUserFormData, type UpdateUserFormData } from "@/lib/validators";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { useCreateUser, useUpdateUser, useUser } from "@/hooks/use-users";
import { toast } from "@/components/ui/toast";

interface UserFormProps {
  userId?: string;
  onSuccess?: () => void;
  onCancel?: () => void;
}

export function UserForm({ userId, onSuccess, onCancel }: UserFormProps) {
  const isEditing = !!userId;
  const { data: existingUser } = useUser(userId || "");
  const createMutation = useCreateUser();
  const updateMutation = useUpdateUser();

  type FormData = { email: string; first_name: string; last_name: string; password?: string };

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormData>({
    resolver: zodResolver(isEditing ? updateUserSchema : createUserSchema) as never,
    values: isEditing && existingUser ? {
      email: existingUser.email,
      first_name: existingUser.first_name,
      last_name: existingUser.last_name,
    } : undefined,
  });

  const onSubmit = (data: FormData) => {
    if (isEditing && userId) {
      updateMutation.mutate(
        { id: userId, data: { email: data.email, first_name: data.first_name, last_name: data.last_name } },
        { onSuccess: () => { toast({ title: "User updated", variant: "success" }); onSuccess?.(); } }
      );
    } else {
      createMutation.mutate(
        { email: data.email, first_name: data.first_name, last_name: data.last_name, password: data.password || "" },
        { onSuccess: () => { toast({ title: "User created", variant: "success" }); onSuccess?.(); } }
      );
    }
  };

  const isPending = createMutation.isPending || updateMutation.isPending;

  return (
    <Card>
      <CardHeader>
        <CardTitle>{isEditing ? "Edit User" : "Create User"}</CardTitle>
        <CardDescription>
          {isEditing ? "Update user information" : "Add a new user to the system"}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="first_name">First Name</Label>
              <Input id="first_name" {...register("first_name")} placeholder="First name" />
              {errors.first_name && <p className="text-xs text-destructive">{errors.first_name.message}</p>}
            </div>
            <div className="space-y-2">
              <Label htmlFor="last_name">Last Name</Label>
              <Input id="last_name" {...register("last_name")} placeholder="Last name" />
              {errors.last_name && <p className="text-xs text-destructive">{errors.last_name.message}</p>}
            </div>
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" {...register("email")} placeholder="user@example.com" />
            {errors.email && <p className="text-xs text-destructive">{errors.email.message}</p>}
          </div>
          {!isEditing && (
            <div className="space-y-2">
              <Label htmlFor="password">Password</Label>
              <Input id="password" type="password" {...register("password")} placeholder="Min 12 characters" />
              {errors.password && <p className="text-xs text-destructive">{errors.password.message}</p>}
            </div>
          )}
          <div className="flex justify-end gap-2">
            {onCancel && <Button type="button" variant="outline" onClick={onCancel}>Cancel</Button>}
            <Button type="submit" disabled={isPending}>
              {isPending ? "Saving..." : isEditing ? "Update" : "Create"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
